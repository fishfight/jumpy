//! Utilities for attaching entities to other entities.
//!
//! There are two kinds of attachments: [`Attachment`] and [`PlayerBodyAttachment`].
//!
//! [`Attachment`] just makes one entity follow another, with some special handling for sprites, and
//! the [`PlayerBodyAttachment`] specifically matches the player's body movement animation to make
//! sure that, for example, a held item will bob up and down with the player.
use crate::prelude::*;

pub fn install(session: &mut Session) {
    session
        .stages
        .add_system_to_stage(CoreStage::Last, update_player_body_attachments)
        .add_system_to_stage(CoreStage::Last, remove_player_body_attachments)
        .add_system_to_stage(CoreStage::Last, update_attachments);
}

/// Component for attaching an entity to another entity.
///
/// > **Warning:** attachments are not a general-purpose hierarchy system. Generally, only expect an
/// > attachment to work properly if the entity it is attached to is not also attached to another
/// > entity.
///
/// Attachments have special behavior built-in for attaching to other [`Sprite`] or [`AtlasSprite`]
/// entities. When attached to a sprite, the attached item will automatically flip it's offset and
/// sprite flip value when the sprite is flipped if `sync_flip` is  set.
#[derive(Clone, Copy, HasSchema, Debug, Default)]
pub struct Attachment {
    /// The entity to attach to.
    pub entity: Entity,
    /// The offset to the attached entity.
    pub offset: Vec3,
    /// Synchronize [`AtlasSprite`] animation with entity animation
    pub sync_animation: bool,
    /// Synchronize [`Sprite`] color with entity color
    pub sync_color: bool,
    /// Synchronize [`Sprite`] flip values from parent (both flip and offset)
    pub sync_flip: bool,
    /// Should attachment inherit parent's rotation (applied to offset and sprite)
    pub offset_inherits_rotation: bool,
}

/// Component that animates attachment offset with easing
#[derive(Clone, HasSchema, Debug, Default)]
pub struct AttachmentEasing {
    /// Initial offset of entity
    pub initial_offset: Vec2,
    /// Target offset after easing is complete
    pub target_offset: Vec2,

    /// Offset ease timer used to update progress of Ease
    pub offset_ease_timer: Timer,

    /// Ease
    pub ease: Ease,
}

/// Metadata for attachment
#[derive(HasSchema, Clone, Default)]
#[type_data(metadata_asset("attachment"))]
#[repr(C)]
pub struct AttachmentMeta {
    pub name: Ustr,
    pub atlas: Handle<Atlas>,
    pub offset: Vec2,
    pub animation: AnimatedSprite,

    /// Synchronize [`AtlasSprite`] animation with entity animation
    pub sync_animation: bool,
    /// Synchronize [`Sprite`] color with entity color
    pub sync_color: bool,
    /// Synchronize [`Sprite`] flip values from parent (both flip and offset)
    pub sync_flip: bool,
    /// Should attachment inherit parent's rotation (applied to offset and sprite)
    pub offset_inherits_rotation: bool,

    pub attachment_easing: Handle<AttachmentEasingMeta>,
}

impl AttachmentMeta {
    /// Make [`Attachment`] from meta and entity
    pub fn attachment(&self, entity: Entity, offset_z: f32) -> Attachment {
        Attachment {
            entity,
            offset: self.offset.extend(offset_z),
            sync_animation: self.sync_animation,
            sync_color: self.sync_color,
            sync_flip: self.sync_flip,
            offset_inherits_rotation: self.offset_inherits_rotation,
        }
    }
}

/// Metadata for [`AttachmentEasing`]
#[derive(HasSchema, Clone, Default)]
#[type_data(metadata_asset("attachment_easing"))]
#[repr(C)]
pub struct AttachmentEasingMeta {
    /// Amount offset is changed over time (target offset = initial_offset + delta_offset)
    pub delta_offset: Vec2,
    /// How long to apply ease function until delta_offset fully applied
    pub offset_ease_duration: Duration,

    pub ease: EaseMeta,
}

impl AttachmentEasing {
    /// Ease progress with delta time
    pub fn tick(&mut self, delta_time: Duration) {
        self.offset_ease_timer.tick(delta_time);
        self.ease.progress = self.offset_ease_timer.percent();
    }
    /// Compute offset from current progress
    pub fn compute_offset(&self) -> Vec2 {
        let progress = self.ease.output();
        self.initial_offset.lerp(self.target_offset, progress)
    }
}

/// System to update the transforms of entities with the [`Attachment`] component.
pub fn update_attachments(
    time: Res<Time>,
    entities: Res<Entities>,
    mut sprites: CompMut<Sprite>,
    attachments: Comp<Attachment>,
    mut attachment_easings: CompMut<AttachmentEasing>,
    invincibles: Comp<Invincibility>,
    mut transforms: CompMut<Transform>,
    mut atlas_sprites: CompMut<AtlasSprite>,
) {
    for (ent, (attachment, easing)) in
        entities.iter_with((&attachments, &mut OptionalMut(&mut attachment_easings)))
    {
        let Some(attached_transform) = transforms.get(attachment.entity).copied() else {
            continue;
        };

        let mut offset = attachment.offset;

        // If have a AttachmentEasing component, update offset
        if let Some(easing) = easing {
            easing.tick(time.delta());
            offset = easing.compute_offset().extend(offset.z);
        }

        let transform = transforms
            .get_mut(ent)
            .expect("Entities with `Attachment` component must also have a `Transform` component.");

        if attachment.offset_inherits_rotation {
            *transform = attached_transform;
        } else {
            // Copy transform values except for rotation
            transform.translation = attached_transform.translation;
            transform.scale = attached_transform.scale;
        }

        if attachment.sync_flip {
            if let Some((flip_x, flip_y)) = atlas_sprites
                .get(attachment.entity)
                .map(|x| (x.flip_x, x.flip_y))
                .or_else(|| sprites.get(attachment.entity).map(|x| (x.flip_x, x.flip_y)))
            {
                if flip_x {
                    offset.x *= -1.0;
                }
                if flip_y {
                    offset.y *= -1.0;
                }

                if let Some((self_flip_x, self_flip_y)) = atlas_sprites
                    .get_mut(ent)
                    .map(|x| (&mut x.flip_x, &mut x.flip_y))
                    .or_else(|| sprites.get_mut(ent).map(|x| (&mut x.flip_x, &mut x.flip_y)))
                {
                    *self_flip_x = flip_x;
                    *self_flip_y = flip_y;
                }
            }
        }

        // Sync animation of attached entity
        if attachment.sync_animation {
            if let Some((index, attach_atlas)) = atlas_sprites
                .get(attachment.entity)
                .map(|atlas| atlas.index)
                .zip(atlas_sprites.get_mut(ent))
            {
                attach_atlas.index = index
            }
        }

        // Sync color of attached entity
        if attachment.sync_color {
            let mut sync_sprite_colors = |alpha| {
                if let Some(entity_sprite) = atlas_sprites.get_mut(ent) {
                    entity_sprite.color.set_a(alpha);
                }

                if let Some(attachment_sprite) = atlas_sprites.get_mut(attachment.entity) {
                    attachment_sprite.color.set_a(alpha);
                }
            };

            match invincibles.get(attachment.entity) {
                None => sync_sprite_colors(1.0),
                Some(_) => sync_sprite_colors(sine_between(
                    *INVINCIBILITY_ALPHA_RANGE.start(),
                    *INVINCIBILITY_ALPHA_RANGE.end(),
                    (time.elapsed().as_millis() / 150) as f32,
                )),
            }
        }

        if attachment.offset_inherits_rotation {
            // Apply parent rotation to offset. Currently scale is not applied (but probably should be.)
            offset = attached_transform.rotation * offset;
        }

        transform.translation += offset;
    }
}

/// A component for attaching an entity to the player's body.
///
/// This is similar to the [`Attachment`] component, but it is special in the way that it will
/// follow the body as it bobs up and down in animations such as standing and walking. This makes it
/// useful for things like hats, etc., that will stick to the player's body.
#[derive(Clone, HasSchema, Default)]
pub struct PlayerBodyAttachment {
    /// The player to attach to.
    pub player: Entity,
    /// The offset relative to the center of the player's sprite.
    pub offset: Vec3,
    /// Whether the attachment should be to the head instead of the body ( i.e. like a hat ).
    pub head: bool,
    /// Whether or not to automatically play the same animation bank animation as the sprite that it
    /// is attached to.
    pub sync_animation: bool,
    /// Whether or not to automatically sync the color of the attached entity with the player's
    pub sync_color: bool,
}

impl PlayerBodyAttachment {
    /// Create a new body attachment to the given player.
    pub fn new(player: Entity) -> Self {
        Self {
            player,
            offset: Vec3::ZERO,
            head: false,
            sync_animation: true,
            sync_color: true,
        }
    }
}

/// This is used by the [`update_player_body_attachments`] system internally.
///
/// It keeps track whether or not an entity had a [`PlayerBodyAttachment`] on the last frame.
#[derive(Clone, Copy, HasSchema, Default)]
struct HadPlayerBodyAttachmentMarker;

/// System that updates entities with the [`PlayerBodyAttachment`] component.
fn update_player_body_attachments(
    entities: Res<Entities>,
    mut attachments: CompMut<Attachment>,
    mut player_body_attachment_markers: CompMut<HadPlayerBodyAttachmentMarker>,
    animated_sprites: Comp<AnimatedSprite>,
    animation_banks: Comp<AnimationBankSprite>,
    player_body_attachments: Comp<PlayerBodyAttachment>,
    player_inputs: Res<MatchInputs>,
    player_indexes: Comp<PlayerIdx>,
    assets: Res<AssetServer>,
) {
    for (ent, body_attachment) in entities.iter_with(&player_body_attachments) {
        let player_ent = body_attachment.player;
        let player_idx = player_indexes.get(player_ent).unwrap();
        let player_handle = player_inputs.players[player_idx.0 as usize].selected_player;
        let meta = assets.get(player_handle);
        let player_sprite = animated_sprites.get(player_ent).unwrap();
        let current_frame = player_sprite.index;
        let current_anim = animation_banks.get(player_ent).unwrap().current;

        let current_body_offset =
            meta.layers.body.animations.offsets[&current_anim][current_frame as usize].body
                + if body_attachment.head {
                    meta.layers.body.animations.offsets[&current_anim][current_frame as usize].head
                } else {
                    Vec2::ZERO
                };

        player_body_attachment_markers.insert(ent, HadPlayerBodyAttachmentMarker);
        attachments.insert(
            ent,
            Attachment {
                entity: player_ent,
                sync_color: body_attachment.sync_color,
                sync_animation: body_attachment.sync_animation,
                offset: current_body_offset.extend(0.0) + body_attachment.offset,
                sync_flip: true,
                offset_inherits_rotation: true,
            },
        );
    }
}

/// System that cleans up old [`Attachment`] and [`HadPlayerBodyAttachmentMarker`] components from
/// an entity when the [`PlayerBodyAttachment`] is removed.
fn remove_player_body_attachments(
    entities: Res<Entities>,
    player_body_attachments: Comp<PlayerBodyAttachment>,
    mut had_player_body_attachment_markers: CompMut<HadPlayerBodyAttachmentMarker>,
    mut attachments: CompMut<Attachment>,
) {
    let mut bitset = had_player_body_attachment_markers.bitset().clone();
    bitset.bit_andnot(player_body_attachments.bitset());

    for entity in entities.iter_with_bitset(&bitset) {
        attachments.remove(entity);
        had_player_body_attachment_markers.remove(entity);
    }
}

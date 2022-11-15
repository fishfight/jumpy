const MapMeta: BevyType<unknown> = {
  typeName: "jumpy::metadata::map::MapMeta",
};

let i = 0;

const CRABS = "crabs";

export default {
  preUpdateInGame() {
    const mapQuery = world.query(MapMeta)[0];
    if (!mapQuery) {
      Script.clearEntityList(CRABS);
      return;
    }

    const spawnedEntities = MapElement.getSpawnedEntities();
    if (spawnedEntities.length > 0) {
      Script.clearEntityList(CRABS);
    }

    // Handle newly spawned map entities
    for (const spanwer_entity of spawnedEntities) {
      const [transform, global_transform, computed_visibility] = world
        .query(Transform, GlobalTransform, ComputedVisibility)
        .get(spanwer_entity);

      // Spawn a new entity for the crab and copy the transform and visibility from the map element
      const entity = WorldTemp.spawn();
      Script.addEntityToList(CRABS, entity);

      world.insert(entity, Value.create(EntityName, ["Critter: Crab"]));
      world.insert(entity, transform);
      world.insert(entity, global_transform);
      world.insert(entity, computed_visibility);
      world.insert(entity, Value.create(Visibility));

      // Add the animated sprite
      world.insert(
        entity,
        Value.create(AnimatedSprite, {
          start: 0,
          end: 1,
          repeat: true,
          fps: 3,
          atlas: {
            id: Assets.getHandleId("crab.atlas.yaml"),
          },
        })
      );

      // And the kinematic body
      world.insert(
        entity,
        Value.create(KinematicBody, {
          size: {
            x: 18,
            y: 12,
          },
          gravity: 1,
          has_friction: true,
          has_mass: true,
        })
      );
    }
  },

  updateInGame() {
    i++;
    const query = world.query(KinematicBody);

    for (const crab of Script.getEntityList(CRABS)) {
      const components = query.get(crab);
      if (!components) continue;
      const [kinematicBody] = components;

      if (i % 100 == 0) {
        i = 0;
        kinematicBody.velocity.x =
          Random.gen() * 3 * (Random.gen() >= 0.5 ? -1 : 1);
      }
    }
  },
};

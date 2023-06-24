use std::time::Duration;

use crate::networking::{
    lan,
    online::{OnlineMatchmakerRequest, OnlineMatchmakerResponse, ONLINE_MATCHMAKER},
    NetworkMatchSocket,
};

use super::*;

#[derive(SystemParam)]
pub struct MatchmakingMenu<'w, 's> {
    time: Res<'w, Time>,
    menu_page: ResMut<'w, MenuPage>,
    game: Res<'w, GameMeta>,
    localization: Res<'w, Localization>,
    state: Local<'s, State>,
    menu_input: Query<'w, 's, &'static mut ActionState<MenuAction>>,
    commands: Commands<'w, 's>,
    storage: ResMut<'w, Storage>,
}

pub struct State {
    match_kind: MatchKind,
    lan_service_discovery_recv: Option<mdns_sd::Receiver<mdns_sd::ServiceEvent>>,
    service_info: Option<mdns_sd::ServiceInfo>,
    status: Status,
    joined_players: usize,
    lan_servers: Vec<lan::ServerInfo>,
    ping_update_timer: Timer,
}

#[derive(Default, PartialEq, Eq)]
pub enum Status {
    #[default]
    Idle,
    Joining,
    Hosting,
    Searching,
}

impl Default for State {
    fn default() -> Self {
        Self {
            match_kind: default(),
            lan_service_discovery_recv: default(),
            service_info: default(),
            status: default(),
            lan_servers: default(),
            joined_players: default(),
            ping_update_timer: Timer::new(Duration::from_secs(1), TimerMode::Repeating),
        }
    }
}

#[derive(Clone)]
pub enum MatchKind {
    Lan(LanMode),
    Online(OnlineState),
}

impl Default for MatchKind {
    fn default() -> Self {
        MatchKind::Lan(LanMode::Join)
    }
}

#[derive(Default, Clone)]
pub enum LanMode {
    #[default]
    Join,
    Host {
        service_name: String,
        player_count: usize,
    },
}

#[derive(Eq, PartialEq, Clone)]
pub struct OnlineState {
    player_count: usize,
    matchmaking_server: String,
    search_state: SearchState,
}

impl Default for OnlineState {
    fn default() -> Self {
        Self {
            player_count: 2,
            matchmaking_server: String::new(),
            search_state: default(),
        }
    }
}

#[derive(Default, PartialEq, Eq, Clone, Copy)]
pub enum SearchState {
    #[default]
    Connecting,
    Searching,
    WaitingForPlayers(usize),
}

impl<'w, 's> WidgetSystem for MatchmakingMenu<'w, 's> {
    type Args = ();
    fn system(
        world: &mut World,
        state: &mut SystemState<Self>,
        ui: &mut egui::Ui,
        _: WidgetId,
        _: (),
    ) {
        let mut params: MatchmakingMenu = state.get_mut(world);
        let menu_input = params.menu_input.single();
        params.state.ping_update_timer.tick(params.time.delta());

        let bigger_text_style = &params.game.ui_theme.font_styles.bigger;
        let normal_text_style = &params.game.ui_theme.font_styles.normal;
        let smaller_text_style = &params.game.ui_theme.font_styles.smaller;
        let heading_text_style = &params.game.ui_theme.font_styles.heading;
        let normal_button_style = &params.game.ui_theme.button_styles.normal;
        let small_button_style = &params.game.ui_theme.button_styles.small;

        ui.vertical_centered(|ui| {
            ui.add_space(heading_text_style.size / 4.0);
            ui.themed_label(heading_text_style, &params.localization.get("network-game"));
            ui.themed_label(
                bigger_text_style,
                &params.localization.get("configure-match"),
            );
            ui.add_space(heading_text_style.size * 4.0);
        });

        let available_size = ui.available_size();
        let x_margin = available_size.x / 4.0;
        let outer_margin = egui::style::Margin::symmetric(x_margin, 0.0);

        BorderedFrame::new(&params.game.ui_theme.panel.border)
            .margin(outer_margin)
            .padding(params.game.ui_theme.panel.padding.into())
            .show(ui, |ui| {
                ui.set_width(ui.available_width());

                ui.horizontal(|ui| {
                    // Lan tab
                    let mut lan = egui::RichText::new(params.localization.get("lan"));
                    if matches!(params.state.match_kind, MatchKind::Lan(..)) {
                        lan = lan.underline();
                    }
                    if BorderedButton::themed(normal_button_style, lan)
                        .show(ui)
                        .clicked()
                    {
                        params.state.match_kind = MatchKind::Lan(default());
                    }

                    // Online tab
                    let mut online = egui::RichText::new(params.localization.get("online"));
                    if matches!(params.state.match_kind, MatchKind::Online(..)) {
                        online = online.underline();
                    }
                    if BorderedButton::themed(normal_button_style, online)
                        .show(ui)
                        .clicked()
                    {
                        params.state.match_kind = MatchKind::Online(default());
                    }

                    match &mut params.state.match_kind {
                        MatchKind::Lan(mode) => {
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    ui.horizontal(|ui| {
                                        // Host tab
                                        let mut host =
                                            egui::RichText::new(params.localization.get("host"));
                                        if matches!(mode, LanMode::Host { .. }) {
                                            host = host.underline();
                                        }
                                        if BorderedButton::themed(
                                            &params.game.ui_theme.button_styles.small,
                                            host,
                                        )
                                        .show(ui)
                                        .clicked()
                                        {
                                            *mode = LanMode::Host {
                                                service_name: params.localization.get("fish-fight"),
                                                player_count: 2,
                                            };
                                        }

                                        // Join tab
                                        let mut join =
                                            egui::RichText::new(params.localization.get("join"));
                                        if matches!(mode, LanMode::Join) {
                                            join = join.underline();
                                        }
                                        if BorderedButton::themed(
                                            &params.game.ui_theme.button_styles.small,
                                            join,
                                        )
                                        .show(ui)
                                        .clicked()
                                        {
                                            *mode = LanMode::Join
                                        }
                                    });
                                },
                            );
                        }
                        MatchKind::Online(_online_state) => {
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    ui.themed_label(
                                        normal_text_style,
                                        &params.localization.get("search-for-match"),
                                    );
                                },
                            );
                        }
                    }
                });

                let State {
                    match_kind,
                    lan_service_discovery_recv,
                    lan_servers,
                    service_info: host_info,
                    status,
                    ping_update_timer,
                    joined_players,
                } = &mut *params.state;

                ui.separator();
                ui.add_space(normal_text_style.size);

                match match_kind {
                    // LAN game
                    MatchKind::Lan(mode) => match mode {
                        LanMode::Join => {
                            // Stop any running server
                            if let Some(service_info) = host_info.take() {
                                lan::stop_server(&service_info);
                                *status = Status::Idle;
                            }
                            lan::prepare_to_join(lan_servers, lan_service_discovery_recv, ping_update_timer);

                            if *status != Status::Joining {
                                ui.themed_label(
                                    normal_text_style,
                                    &params.localization.get("servers"),
                                );
                                ui.add_space(normal_text_style.size / 2.0);

                                ui.indent("servers", |ui| {
                                    for server in lan_servers.iter() {
                                        ui.horizontal(|ui| {
                                            if BorderedButton::themed(
                                                &params.game.ui_theme.button_styles.normal,
                                                server.service.get_hostname().trim_end_matches('.'),
                                            )
                                            .min_size(egui::vec2(ui.available_width() * 0.8, 0.0))
                                            .show(ui)
                                            .clicked()
                                            {
                                                lan::join_server(server);
                                                *status = Status::Joining;
                                            }

                                            let label_text = egui::RichText::new(format!(
                                                "🖧 {}ms",
                                                server
                                                    .ping
                                                    .map(|x| x.to_string())
                                                    .unwrap_or("?".into())
                                            ))
                                            .size(normal_text_style.size);
                                            ui.label(label_text)
                                        });
                                    }

                                    if lan_servers.is_empty() {
                                        ui.themed_label(
                                            normal_text_style,
                                            &params.localization.get("no-servers"),
                                        );
                                    }
                                });

                            // If we are trying to join a match.
                            } else {
                                ui.themed_label(
                                    normal_text_style,
                                    &params.localization.get("joining"),
                                );

                                if let Some(lan_socket) = lan::wait_game_start() {
                                    params.commands.insert_resource(NetworkMatchSocket(
                                        Box::new(lan_socket),
                                    ));

                                    *status = default();
                                    *params.menu_page = MenuPage::PlayerSelect;
                                }
                            }

                            ui.add_space(normal_text_style.size / 2.0);
                        }
                        LanMode::Host {
                            service_name,
                            player_count,
                        } => {
                            ui.scope(|ui| {
                                ui.set_enabled(*status != Status::Hosting);
                                ui.horizontal(|ui| {
                                    ui.themed_label(
                                        normal_text_style,
                                        &params.localization.get("server-name"),
                                    );
                                    ui.add(
                                        egui::TextEdit::singleline(service_name)
                                            .font(normal_text_style.font_id()),
                                    );
                                    *service_name = service_name.replace(' ', "-");
                                });
                                ui.add_space(normal_text_style.size / 2.0);
                                ui.horizontal(|ui| {
                                    ui.themed_label(
                                        normal_text_style,
                                        &params.localization.get("player-count"),
                                    );
                                    ui.add_space(normal_text_style.size);
                                    ui.scope(|ui| {
                                        ui.set_enabled(*player_count > 2);
                                        if BorderedButton::themed(small_button_style, "-")
                                            .min_size(egui::vec2(normal_text_style.size * 2.0, 0.0))
                                            .show(ui)
                                            .clicked()
                                        {
                                            *player_count = player_count
                                                .saturating_sub(1)
                                                .clamp(2, MAX_PLAYERS);
                                        }
                                    });
                                    ui.themed_label(normal_text_style, &player_count.to_string());
                                    ui.scope(|ui| {
                                        ui.set_enabled(*player_count < MAX_PLAYERS);
                                        if BorderedButton::themed(small_button_style, "+")
                                            .min_size(egui::vec2(normal_text_style.size * 2.0, 0.0))
                                            .show(ui)
                                            .clicked()
                                        {
                                            *player_count = player_count
                                                .saturating_add(1)
                                                .clamp(2, MAX_PLAYERS);
                                        }
                                    });

                                    *service_name = service_name.replace(' ', "-");
                                });
                            });

                            let (is_recreated, service_info) = lan::prepare_to_host(host_info, service_name);
                            if is_recreated {
                                *status = Status::Idle;
                            }

                            ui.add_space(params.game.ui_theme.font_styles.normal.size);

                            if *status == Status::Idle {
                                if BorderedButton::themed(
                                    small_button_style,
                                    &params.localization.get("start-server"),
                                )
                                .show(ui)
                                .clicked()
                                {
                                    *status = Status::Hosting;
                                    lan::start_server(service_info.clone(), *player_count);
                                }

                            // If we are hosting a match currently
                            } else if *status == Status::Hosting {
                                if let Some(lan_socket) = lan::wait_players(joined_players, service_info) {
                                    params.commands.insert_resource(NetworkMatchSocket(
                                        Box::new(lan_socket),
                                    ));

                                    *status = default();
                                    *params.menu_page = MenuPage::PlayerSelect;
                                }

                                ui.horizontal(|ui| {
                                    if BorderedButton::themed(
                                        small_button_style,
                                        &params.localization.get("stop-server"),
                                    )
                                    .show(ui)
                                    .clicked()
                                    {
                                        lan::stop_server(service_info);
                                        *status = Status::Idle;
                                    }

                                    ui.themed_label(
                                        normal_text_style,
                                        &format!(
                                            "{} {} / {}",
                                            &params.localization.get("players"),
                                            *joined_players + 1, // Add one to count the host.
                                            player_count
                                        ),
                                    );
                                });
                            }
                        }
                    },

                    // Online game
                    MatchKind::Online(OnlineState {
                        player_count,
                        matchmaking_server,
                        mut search_state,
                    }) => {
                        // Get the matchmaking server from the settings.
                        if matchmaking_server.is_empty() {
                            *matchmaking_server = params
                                .storage
                                .get::<Settings>(Settings::STORAGE_KEY)
                                .unwrap_or_else(|| params.game.default_settings.clone())
                                .matchmaking_server;
                        }

                        ui.horizontal(|ui| {
                            ui.set_enabled(*status == Status::Idle);
                            ui.themed_label(
                                normal_text_style,
                                &params.localization.get("player-count"),
                            );

                            ui.scope(|ui| {
                                ui.set_enabled(*player_count > 2);
                                if BorderedButton::themed(small_button_style, "-")
                                    .min_size(egui::vec2(normal_text_style.size * 2.0, 0.0))
                                    .show(ui)
                                    .clicked()
                                {
                                    *player_count =
                                        player_count.saturating_sub(1).clamp(2, MAX_PLAYERS);
                                }
                            });
                            ui.themed_label(normal_text_style, &player_count.to_string());
                            ui.scope(|ui| {
                                ui.set_enabled(*player_count < MAX_PLAYERS);
                                if BorderedButton::themed(small_button_style, "+")
                                    .min_size(egui::vec2(normal_text_style.size * 2.0, 0.0))
                                    .show(ui)
                                    .clicked()
                                {
                                    *player_count =
                                        player_count.saturating_add(1).clamp(2, MAX_PLAYERS);
                                }
                            });
                        });

                        ui.add_space(normal_text_style.size);

                        if *status == Status::Idle {
                            if BorderedButton::themed(
                                small_button_style,
                                &params.localization.get("search"),
                            )
                            .show(ui)
                            .clicked()
                            {
                                *status = Status::Searching;
                                ONLINE_MATCHMAKER
                                    .try_send(OnlineMatchmakerRequest::SearchForGame {
                                        addr: matchmaking_server.clone(),
                                        player_count: *player_count,
                                    })
                                    .unwrap();
                            }
                        } else if *status == Status::Searching {
                            while let Ok(message) = ONLINE_MATCHMAKER.try_recv() {
                                match message {
                                    OnlineMatchmakerResponse::Searching => {
                                        search_state = SearchState::Searching
                                    }
                                    OnlineMatchmakerResponse::PlayerCount(count) => {
                                        search_state = SearchState::WaitingForPlayers(count)
                                    }
                                    OnlineMatchmakerResponse::GameStarting {
                                        online_socket,
                                        player_idx,
                                        player_count: _,
                                    } => {
                                        info!(?player_idx, "Starting network game");
                                        params.commands.insert_resource(NetworkMatchSocket(
                                            Box::new(online_socket),
                                        ));

                                        *status = default();
                                        search_state = default();
                                        *params.menu_page = MenuPage::PlayerSelect;
                                    }
                                }
                            }

                            ui.horizontal(|ui| {
                                if BorderedButton::themed(
                                    small_button_style,
                                    &params.localization.get("cancel"),
                                )
                                .show(ui)
                                .clicked()
                                {
                                    ONLINE_MATCHMAKER
                                        .try_send(OnlineMatchmakerRequest::StopSearch)
                                        .unwrap();
                                    search_state = default();
                                    *status = Status::Idle;
                                }

                                ui.themed_label(
                                    smaller_text_style,
                                    &match search_state {
                                        SearchState::Connecting => {
                                            params.localization.get("connecting")
                                        }
                                        SearchState::Searching => {
                                            params.localization.get("searching")
                                        }
                                        SearchState::WaitingForPlayers(current) => {
                                            params.localization.get(&format!(
                                                "waiting-for-players?current={current}&total={player_count}",
                                            ))
                                        }
                                    },
                                );
                            });
                        }
                    }
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                    if BorderedButton::themed(normal_button_style, &params.localization.get("back"))
                        .show(ui)
                        .clicked()
                        || menu_input.pressed(MenuAction::Back)
                    {
                        match status {
                            Status::Idle => (),
                            Status::Searching => {
                                if let Err(err) = ONLINE_MATCHMAKER.try_send(OnlineMatchmakerRequest::StopSearch){
                                    error!("Error stopping search: {:?}", err);
                                }

                                *status = Status::Idle;
                            }
                            Status::Joining => {
                                lan::leave_server();
                                *status = Status::Idle;
                            }
                            Status::Hosting => {
                                if let Some(service_info) = host_info.take() {
                                    lan::stop_server(&service_info);
                                }
                                *status = Status::Idle;
                            }
                        }
                        *params.menu_page = MenuPage::Home;
                    }
                });
            });
    }
}

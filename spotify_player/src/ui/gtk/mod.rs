use crate::{client::ClientRequest, state::SharedState};
use glib;
use gtk::prelude::*;
use gtk4 as gtk;
use std::sync::Arc;

const STYLE: &str = include_str!("style.css");

pub fn run(state: &SharedState, client_pub: flume::Sender<ClientRequest>) {
    let app = gtk::Application::builder()
        .application_id("com.github.aome510.spotify_player")
        .build();

    let state = Arc::clone(state);

    app.connect_activate(move |app| {
        let provider = gtk::CssProvider::new();
        provider.load_from_data(STYLE);
        gtk::style_context_add_provider_for_display(
            &gtk::gdk::Display::default().expect("Could not connect to a display."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
        build_ui(app, &state, &client_pub);
    });

    app.run_with_args::<&str>(&[]);
}

fn build_ui(
    app: &gtk::Application,
    state: &SharedState,
    client_pub: &flume::Sender<ClientRequest>,
) {
    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .title("Spotify Player (Winamp Edition)")
        .default_width(400)
        .default_height(200)
        .build();

    let main_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
    main_box.set_widget_name("main-window");
    window.set_child(Some(&main_box));

    // Playback info
    let info_box = gtk::Box::new(gtk::Orientation::Vertical, 5);
    info_box.set_widget_name("info-box");
    main_box.append(&info_box);

    let title_label = gtk::Label::new(Some("No Track Playing"));
    title_label.set_widget_name("track-title");
    info_box.append(&title_label);

    let artist_label = gtk::Label::new(Some("Unknown Artist"));
    artist_label.set_widget_name("track-artist");
    info_box.append(&artist_label);

    // Progress bar
    let progress_bar = gtk::ProgressBar::new();
    progress_bar.set_widget_name("playback-progress");
    main_box.append(&progress_bar);

    // Controls
    let controls_box = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    controls_box.set_halign(gtk::Align::Center);
    controls_box.set_widget_name("controls-box");
    main_box.append(&controls_box);

    let prev_button = gtk::Button::with_label("PREV");
    let play_pause_button = gtk::Button::with_label("PLAY");
    let next_button = gtk::Button::with_label("NEXT");

    controls_box.append(&prev_button);
    controls_box.append(&play_pause_button);
    controls_box.append(&next_button);

    // Connect button signals
    let cp = client_pub.clone();
    prev_button.connect_clicked(move |_| {
        let _ = cp.send(ClientRequest::Player(
            crate::client::PlayerRequest::PreviousTrack,
        ));
    });

    let cp = client_pub.clone();
    play_pause_button.connect_clicked(move |_| {
        let _ = cp.send(ClientRequest::Player(
            crate::client::PlayerRequest::ResumePause,
        ));
    });

    let cp = client_pub.clone();
    next_button.connect_clicked(move |_| {
        let _ = cp.send(ClientRequest::Player(
            crate::client::PlayerRequest::NextTrack,
        ));
    });

    // Update UI state
    let state_clone = state.clone();
    glib::timeout_add_local(std::time::Duration::from_millis(500), move || {
        update_ui(
            &state_clone,
            &title_label,
            &artist_label,
            &progress_bar,
            &play_pause_button,
        );
        glib::ControlFlow::Continue
    });

    window.present();
}

fn update_ui(
    state: &SharedState,
    title_label: &gtk::Label,
    artist_label: &gtk::Label,
    progress_bar: &gtk::ProgressBar,
    play_pause_button: &gtk::Button,
) {
    let player = state.player.read();
    if let Some(playback) = player.current_playback() {
        if let Some(ref item) = playback.item {
            match item {
                rspotify::model::PlayableItem::Track(track) => {
                    title_label.set_text(&track.name);
                    artist_label.set_text(
                        &track
                            .artists
                            .iter()
                            .map(|a| a.name.as_str())
                            .collect::<Vec<_>>()
                            .join(", "),
                    );
                }
                rspotify::model::PlayableItem::Episode(episode) => {
                    title_label.set_text(&episode.name);
                    artist_label.set_text(&episode.show.name);
                }
                _ => {
                    title_label.set_text("Unknown Track");
                    artist_label.set_text("Unknown Artist");
                }
            }
        }

        if let (Some(progress), Some(duration)) = (
            playback.progress,
            playback.item.as_ref().and_then(|i| match i {
                rspotify::model::PlayableItem::Track(t) => Some(t.duration),
                rspotify::model::PlayableItem::Episode(e) => Some(e.duration),
                _ => None,
            }),
        ) {
            let fraction = progress.num_milliseconds() as f64 / duration.num_milliseconds() as f64;
            progress_bar.set_fraction(fraction);
        }

        if playback.is_playing {
            play_pause_button.set_label("PAUSE");
        } else {
            play_pause_button.set_label("PLAY");
        }
    }
}

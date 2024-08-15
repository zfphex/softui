```rs
//Declare icons, constants, set font etc...

//Top icons
let mut close = button(CLOSE).wh(48).radius(24).left(0);
let mut audio = button(AUDIO).wh(48).radius(24).right(0);

horizontal(&mut close, &mut audio);

if close.clicked() {
    //...
}

if audio.clicked() {
    //...
}

let mut prev = button(PREV);
let mut play_pause = if player.paused { button(PAUSED) } else { button(PLAY) };
let mut next = button(NEXT);
let mut progress = progress_bar(remaining / elapsed).radius(12).fg(WHITE).bg(BLACK);
let mut settings = button(SETTINGS);
let mut audio_settings = button(AUDIO_SETTINGS);

//Bottom bar
horizontal(
    &mut prev,
    &mut play_pause
    &mut next,
    text(elapsed), 
    &mut progress
    &mut settings
).width(0.80).height(0.10).radius(12).bg(DARK_GREY);

if prev.clicked() {
    player.prev();
}

if play_pause.clicked() {
    if player.paused {
        player.play();
    } else {
        player.pause();
    }
}

if next.clicked() {
    player.next();
}

if progress.clicked(progress) {
    player.seek(progress.position)
}

if settings.clicked() {
    show_settings = true;
}

if audio_settings.clicked() {
    show_audio_settings = true;
}


if !show_audio_settings {
    break;
}

//Constants etc...

fn item(icon: Icon, header: &str, footer: &str) -> Button {
    button(
        h(icon, v(text(header), text(footer).font_size(12).fg(FONT_SUB)))
    ).padding_left(12).bg(SUB_HEADING)
}

let mut langauges = Vec::new();

for (language: &str, codec: &str) in _ {
    langauges.push(item(AUDIO_ICON, language, codec));
}

let mut subtitles = Vec::new();
for (subtitle: &str, language: &str) in _ {
    subtitles.push(item(SUBTITLE_ICON, subtitle, language));
}

v(
    "Audio".bg(HEADING), // Ideally text should implement view or whatever.
    &mut langauges,
    text("Subtitles").bg(HEADING),
    &mut subtitles
).w(0.25).h(0.5)

if languages.clicked(index) {
    current_langauge = langauges[index]
}

if subtitles.clicked(index) {
    current_subtitle = subtitles[index]
}
```
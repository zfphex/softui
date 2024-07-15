```rs
const PREV: Icon = todo!();
const PLAY: Icon = todo!();
const NEXT: Icon = todo!();
const SETTINGS: Icon = todo!();
const AUDIO_SETTINGS: Icon = todo!();

const CLOSE: Icon = todo!();
const CHAPTERS: Icon = todo!();

const FONT_PRIMARY: Color = todo!();
const FONT_SUB: Color = todo!();

let mut elapsed = 1420.0;
let mut remaining = 2.0;
let mut show_settings = false;
let mut show_audio_settings = false;

ctx.set_default_font_size(18);
ctx.set_font_color(FONT_PRIMARY);

//Top icons
let b = button().wh(48).radius(24)
//  ^ This would have drawn      |
//If the user had done b.clone() V here.
h(b.clone().icon(CLOSE).left(0), b.icon(AUDIO).right(0));
//Since drop would have been called.
//I'm not sure what to do about that.
//We draw on drop.

//Bottom bar
h(
    button(PREV).clicked(|_| player.prev()), 
    if player.paused { button(PAUSED).clicked(|_| player.play()) } else { button(PLAY).clicked(|_| player.pause()) }, 
    button(NEXT).clicked(|_| player.next()), 
    text(elapsed), 
    progress_bar(remaining / elapsed).radius(12).fg(WHITE).bg(BLACK).clicked(|_, position| player.seek(position)),
    button(SETTINGS).clicked(|_| show_settings = true),
    button(AUDIO_SETTINGS).clicked(|_| show_audio_settings = true),
).width(0.80).height(0.10).radius(12).bg(DARK_GREY)

if !show_audio_settings {
    break;
}

const AUDIO_ICON: Icon = todo!();
const SUBTITLE_ICON: Icon = todo!();
const HEADING: Color = todo!();
const SUB_HEADING: Color = todo!();

let mut current_language = todo!();
let mut current_subtitle = todo!();

fn item(icon: Icon, header: &str, footer: &str) -> Button {
    button(
        h(icon, v(text(header), text(footer).font_size(12).fg(FONT_SUB)))
    ).padding_left(12).bg(SUB_HEADING)
}

let langauges = Vec::new();
for (language: &str, codec: &str) in _ {
    langauges.push(item(AUDIO_ICON, language, codec).clicked(|_| current_language = language));
}

let subtitles = Vec::new();
for (subtitle: &str, language: &str) in _ {
    subtitles.push(item(SUBTITLE_ICON, subtitle, language).clicked(|_| current_subtitle = subtitle));
}

v(
    "Audio".bg(HEADING), // Ideally text should implement view or whatever.
    langauges,
    text("Subtitles").bg(HEADING),
    subtitles
).w(0.25).h(0.5)

```
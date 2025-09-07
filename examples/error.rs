fn main() {
    struct _E {}
    softui::flex!(
        _E {},
        softui::text("this is a test of the layout"),
        softui::text("next widget"),
    );
}

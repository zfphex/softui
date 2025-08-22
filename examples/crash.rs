use softui::*;

fn main() {
    // Top-level layout: LeftRight with
    //  - left: subgroup with two percent children (20% / 80%)
    //  - right: fixed 20px rect
    //
    // Window 800 -> subgroup should receive 780, sibling 20.
    // subgroup children should be 20% of 780 = 156 and 80% of 780 = 624.

    let total_area = Rect::new(0, 0, 800, 600);

    // Build a subgroup and push into the parent
    let mut left = Group::new().direction(FlexDirection::LeftRight);
    left.children.push(Box::new(rect().w(20.percent()).h(20)));
    left.children.push(Box::new(rect().w(80.percent()).h(20)));

    let mut parent = Group::new().direction(FlexDirection::LeftRight);
    parent.children.push(Box::new(left));
    parent.children.push(Box::new(rect().wh(20))); // fixed 20px

    // Size and layout parent
    let size = parent.size(total_area);
    parent.position(size, total_area);

    // Assert parent assigned widths: left = 780, right = 20
    assert_eq!(
        *parent.children[0].area_mut(),
        *rect().x(0).y(0).w(780).h(20).area_mut()
    );
    assert_eq!(
        *parent.children[1].area_mut(),
        *rect().x(780).y(0).w(20).h(20).area_mut()
    );

    // Independently verify subgroup resolves percentages against a 780-wide box
    let mut left2 = Group::new().direction(FlexDirection::LeftRight);
    left2.children.push(Box::new(rect().w(20.percent()).h(20)));
    left2.children.push(Box::new(rect().w(80.percent()).h(20)));

    let left_size = left2.size(Rect::new(0, 0, 780, 600));
    left2.position(left_size, Rect::new(0, 0, 780, 20));

    assert_eq!(*left2.children[0].area_mut(), *rect().x(0).y(0).w(156).h(20).area_mut());
    assert_eq!(
        *left2.children[1].area_mut(),
        *rect().x(156).y(0).w(624).h(20).area_mut()
    );
}

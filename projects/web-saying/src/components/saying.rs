pub use lady_saying::LadySaying;
pub use super::*;

#[derive(PartialEq, Props)]
pub struct SayingProps {
    lady: LadySaying,
}

pub fn Saying(cx: Scope<SayingProps>) -> Element {
    let styling = include_str!("styling.css");
    let rules = cx.props.lady.rule.iter().map(|(k, v)| rsx!(
        tr {
            td {
                class: "key",
                "{k}"
            }
            td {
                class: "change",
                "换成"
            }
            td {
                class: "value",
                "{v}"
            }
        }
));

    cx.render(rsx! {
        div {
            class: "saying",
            style {
                "{styling}"
            }
            div {
                class: "title",
                "女生说话像 {cx.props.lady.like} 的正确方式"
            }
            table {
                rules
            }
        }
    })
}

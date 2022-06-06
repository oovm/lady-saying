

#[derive(PartialEq, Props)]
pub struct Saying {
    lady: LadySaying,
}

pub fn SayingComponent(cx: Scope<Saying>) -> Element {
    cx.render(rsx! {
        div {
            span {
                class: "title",
                "女生说话像 {cx.props.lady.like} 的正确方式"
            }
            span { "+"}
            span { "-" }
        }
    })
}

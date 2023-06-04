use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    pub children: Children,
}

#[function_component]
pub fn Layout(props: &LayoutProps) -> Html {
    html! {
        <div class="prose mx-4 sm:mx-auto my-16">
            <h1>{ "음력 기념일 ICS 생성기" }</h1>
            <section class="flex flex-col sm:flex-row gap-8">
                { for props.children.iter() }
            </section>
            <footer>
                <ul class="list-none px-0 flex justify-center gap-4">
                    <li>{ "© jangjunha" }</li>
                    <li>
                        <a
                            href="https://github.com/jangjunha/lunar-ics-generator-web"
                            target="_blank"
                            class="text-blue-500"
                        >{ "GitHub에서 보기" }</a>
                    </li>
                </ul>
            </footer>
        </div>
    }
}

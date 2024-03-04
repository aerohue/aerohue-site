use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <body>
            <h1>{ "test" }</h1>
            <p>{"Hello world!"}</p>
            <h2>{ "test" }</h2>
            <p>{"test"}</p>
            <img height=256 src = "https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fnationalinterest.org%2Fsites%2Fdefault%2Ffiles%2Fmain_images%2FDouglas_A-4E_Skyhawk_of_VA-164_in_flight_over_Vietnam_on_21_November_1967_(6430101)%2520(1).jpg&f=1&nofb=1&ipt=899f20f3ff076273775a4339184bb89a751597f73a89cc3854ae1ea146240318&ipo=images"/>

            
        </body>
        }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
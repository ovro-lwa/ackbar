use plotly::{Plot, Scatter, ScatterPolar};
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <div>
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
        <PlotComponent />
        </div>
    }
}

#[function_component(PlotComponent)]
pub fn plot_component() -> Html {
    let p = yew_hooks::use_async::<_, _, ()>({
        let id = "plot-div";
        let mut plot = Plot::new();
        let trace = ScatterPolar::new(vec![0, 1, 2], vec![2, 1, 0]);
        plot.add_trace(trace);

        async move {
            plotly::bindings::new_plot(id, &plot).await;
            Ok(())
        }
    });


        use_effect_with_deps(move |_| {
            p.run();
            || ()
        }, (),
    );


    html! {
        <div id="plot-div"></div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}


use druid::im::Vector;
use druid::widget::{
    Button, CrossAxisAlignment, Flex, Label, List, RadioGroup, Scroll, TextBox, WidgetExt,
};
use druid::{
    AppLauncher, ArcStr, Color, Data, Env, EventCtx, Lens, Size, UnitPoint, Widget, WindowDesc,
};
use druid_widget_nursery::{AutoCompleteTextBox, Dropdown, FuzzySearchData, DROP};
use std::sync::Arc;

#[derive(Data, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
enum Fruit {
    Apple,
    Pear,
    Orange,
}

#[derive(Data, Clone, Lens)]
struct DropDownState {
    fruit: Fruit,
    place: String,
    fs: FuzzySearchData,
}

fn main_widget() -> impl Widget<DropDownState> {
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(Label::new("Autocomplete dropdown. "))
        //.with_spacer(10.)
        .with_child(Dropdown::new_sized(
            AutoCompleteTextBox::new(),
            |_, _| {
                Scroll::new(List::new(|| {
                    Label::new(|item: &ArcStr, _env: &_| format!("{}", item)).padding(10.0)
                }))
                .vertical()
                .lens(FuzzySearchData::suggestions)
            },
            Size::from((100., 70.)),
        ))
        .with_child(Label::new("Label that'll be overlapped by dropdown"))
        .with_spacer(80.)
        .with_child(Label::new("Words in BK tree"))
        .with_child(Dropdown::new_sized(
            AutoCompleteTextBox::new(),
            |_, _| {
                Scroll::new(List::new(|| {
                    Label::new(|item: &ArcStr, _env: &_| format!("{}", item)).padding(10.0)
                }))
                    .vertical()
                    .lens(FuzzySearchData::suggestions)
            },
            Size::from((100., 70.)),
        ))
        .with_child(
            Scroll::new(
                List::new(|| Label::new(|item: &String, _env: &_| format!("{}", item)))
                    .lens(FuzzySearchData::existing_words),
            )
            .fix_width(200.)
            .fix_height(200.),
        )
        .lens(DropDownState::fs)
        .debug_paint_layout()
}

pub fn main() {
    let main_window = WindowDesc::new(main_widget())
        .title("Dropdown")
        .window_size((250., 500.));

    // create the initial app state
    let fs = FuzzySearchData {
        word: String::new(),
        suggestions: Arc::new(vec![]),
        tolerance: 3,
        existing_words: Vector::new(),
    };

    // create the initial app state
    let initial_state = DropDownState {
        fruit: Fruit::Apple,
        place: "California".to_owned(),
        fs,
    };

    // start the application
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(initial_state)
        .expect("Failed to launch application");
}

use druid::im::Vector;
use druid::keyboard_types::Key;
use druid::text::{EditableText, TextStorage};
use druid::widget::{
    Button, CrossAxisAlignment, Flex, Label, List, RadioGroup, Scroll, TextBox, WidgetExt,
};

use crate::DROP;
use druid::{
    AppLauncher, ArcStr, BoxConstraints, Color, Data, Env, Event, EventCtx, LayoutCtx, Lens,
    LifeCycle, LifeCycleCtx, PaintCtx, Size, UnitPoint, UpdateCtx, Widget, WindowDesc,
};
use std::borrow::{Borrow, BorrowMut};
use std::sync::Arc;
use LevenshteinDistance::BKTree;

#[derive(Data, Clone, Lens)]
pub struct FuzzySearchData {
    pub word: String,
    pub suggestions: Arc<Vec<ArcStr>>,
    pub tolerance: usize,
}

pub struct AutoCompleteTextBox {
    textbox: TextBox<String>,
    bk_tree: BKTree,
}

impl AutoCompleteTextBox {
    pub fn new() -> Self {
        AutoCompleteTextBox {
            textbox: TextBox::new(),
            bk_tree: BKTree::new(""),
        }
    }
}

impl Widget<FuzzySearchData> for AutoCompleteTextBox {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut FuzzySearchData, env: &Env) {
        self.textbox.event(ctx, event, data.word.borrow_mut(), env);
        match event {
            Event::KeyUp(e) if e.key.ne(&Key::Enter) => {
                if data.word.len() > 3 {
                    //let mut result = self.bk_tree.fuzzy_search(data.word.as_str(), 2);
                    let mut result = vec!["sujit"];
                    println!("size of suggestions {}", data.suggestions.len());
                    //not good to clone. what to do? :(
                    let suggestions = Arc::make_mut(&mut data.suggestions);
                    println!("size of result {}", result.len());
                    //suggestions.clear();
                    result
                        .iter()
                        .for_each(|s| suggestions.push(ArcStr::from(*s)));
                    dbg!("Drop called");
                    ctx.submit_command(DROP);
                }
            }
            Event::KeyUp(e) if e.key.eq(&Key::Enter) => {
                if data.word.len() > 3 {
                    self.bk_tree.add_word(data.word.as_str());
                    println!("Added {} to BKTree", data.word);
                }
            }
            _ => {}
        }
    }

    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &FuzzySearchData,
        env: &Env,
    ) {
        match event {
            LifeCycle::HotChanged(hc) => {
                if *hc {
                    ctx.submit_command(DROP);
                }
            }
            _ => {}
        }
        self.textbox.lifecycle(ctx, event, data.word.borrow(), env);
    }

    fn update(
        &mut self,
        ctx: &mut UpdateCtx,
        old_data: &FuzzySearchData,
        data: &FuzzySearchData,
        env: &Env,
    ) {
        self.textbox
            .update(ctx, old_data.word.borrow(), data.word.borrow(), env);
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &FuzzySearchData,
        env: &Env,
    ) -> Size {
        self.textbox.layout(ctx, bc, data.word.borrow(), env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &FuzzySearchData, env: &Env) {
        self.textbox.paint(ctx, data.word.borrow(), env);
    }
}

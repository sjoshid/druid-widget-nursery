use druid::im::Vector;
use druid::keyboard_types::Key;
use druid::text::{EditableText, TextStorage};
use druid::widget::{
    Button, CrossAxisAlignment, Flex, Label, List, RadioGroup, Scroll, TextBox, WidgetExt,
};

use crate::{CLOSE_DROP, DROP};
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
    pub existing_words: Vector<String>,
}

pub struct AutoCompleteTextBox {
    textbox: TextBox<String>,
    bk_tree: BKTree,
    dropdown_shown: bool,
}

impl AutoCompleteTextBox {
    pub fn new() -> Self {
        AutoCompleteTextBox {
            textbox: TextBox::new(),
            bk_tree: BKTree::new(""),
            dropdown_shown: false,
        }
    }
}

impl Widget<FuzzySearchData> for AutoCompleteTextBox {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut FuzzySearchData, env: &Env) {
        match event {
            Event::KeyUp(e) if e.key.ne(&Key::Enter) => {
                if data.word.len() > 3 {
                    let result = self
                        .bk_tree
                        .fuzzy_search(data.word.as_str(), data.tolerance);
                    dbg!(result.len());
                    let mut result_vec: Vec<ArcStr> = Vec::with_capacity(result.len());
                    for r in result.into_iter() {
                        let s = String::from(r);
                        result_vec.push(s.into());
                    }
                    data.suggestions = Arc::new(result_vec);
                    if !self.dropdown_shown {
                        ctx.submit_command(DROP);
                        self.dropdown_shown = true;
                    }
                }
            }
            Event::KeyUp(e) if e.key.eq(&Key::Enter) => {
                if data.word.len() > 3 {
                    self.bk_tree.add_word(data.word.as_str());
                    data.existing_words.push_back(data.word.clone());
                }
            }
            Event::Timer(id) => {}
            _ => {
                self.textbox.event(ctx, event, data.word.borrow_mut(), env);
            }
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
            LifeCycle::FocusChanged(hc) => {
                if !*hc {
                    ctx.submit_command(CLOSE_DROP);
                    self.dropdown_shown = false;
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

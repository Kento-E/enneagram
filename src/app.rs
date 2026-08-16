use yew::prelude::*;

use crate::models::{AxisSelection, DiagnosisResult, EnneagramType, Rank, StoredResult};
use crate::questions::build_questions;
use crate::storage::{load_result, save_result};

pub struct App {
    questions: Vec<crate::models::AxisQuestion>,
    selections: Vec<AxisSelection>,
    current_axis: usize,
    current_result: Option<DiagnosisResult>,
    stored_result: Option<StoredResult>,
    show_stored_result: bool,
    message: Option<String>,
    show_explanation: bool,
}

pub enum Msg {
    SelectRank { item_index: usize, rank: Rank },
    ClearItem(usize),
    NextAxis,
    PrevAxis,
    Submit,
    ResetForm,
    ShowStoredResult,
    BackToForm,
    CloseExplanation,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let questions = build_questions();
        let selections = vec![AxisSelection::default(); questions.len()];
        let stored_result = load_result();

        Self {
            questions,
            selections,
            current_axis: 0,
            current_result: None,
            stored_result,
            show_stored_result: false,
            message: None,
            show_explanation: true,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SelectRank { item_index, rank } => {
                let selection = &mut self.selections[self.current_axis];
                if rank == Rank::Slight
                    && !selection.slight.contains(&item_index)
                    && selection.slight.len() >= 2
                {
                    self.message = Some("「少しは当てはまる」は2個まで選択できます。".to_string());
                    return true;
                }

                if let Err(err) = selection.assign_rank(rank, item_index) {
                    self.message = Some(err.to_string());
                } else {
                    self.message = None;
                }
                true
            }
            Msg::ClearItem(item_index) => {
                self.selections[self.current_axis].clear_item(item_index);
                self.message = None;
                true
            }
            Msg::NextAxis => {
                if !self.selections[self.current_axis].is_complete() {
                    self.message = Some(
                        "この設問では「最も当てはまる」を1つ、「次に当てはまる」を1つ、「少しは当てはまる」を2つ選んでください。"
                            .to_string(),
                    );
                    return true;
                }

                if self.current_axis + 1 < self.questions.len() {
                    self.current_axis += 1;
                    self.message = None;
                }
                true
            }
            Msg::PrevAxis => {
                if self.current_axis > 0 {
                    self.current_axis -= 1;
                    self.message = None;
                }
                true
            }
            Msg::Submit => {
                if !self.selections[self.current_axis].is_complete() {
                    self.message = Some(
                        "この設問では「最も当てはまる」を1つ、「次に当てはまる」を1つ、「少しは当てはまる」を2つ選んでください。"
                            .to_string(),
                    );
                    return true;
                }

                let result = calculate_result(&self.questions, &self.selections);
                let stored = StoredResult {
                    scores: result.scores,
                    top_types: result.top_types.clone(),
                    generated_at: js_sys::Date::new_0().to_iso_string().into(),
                };
                save_result(&stored);

                self.current_result = Some(result);
                self.stored_result = Some(stored);
                self.show_stored_result = false;
                self.message = None;
                true
            }
            Msg::ResetForm => {
                self.selections = vec![AxisSelection::default(); self.questions.len()];
                self.current_axis = 0;
                self.current_result = None;
                self.show_stored_result = false;
                self.message = None;
                true
            }
            Msg::ShowStoredResult => {
                self.show_stored_result = true;
                self.current_result = None;
                true
            }
            Msg::BackToForm => {
                self.show_stored_result = false;
                true
            }
            Msg::CloseExplanation => {
                self.show_explanation = false;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if self.show_explanation {
            return self.view_explanation(ctx);
        }

        let content = if let Some(result) = self.active_result() {
            self.view_result(ctx, &result)
        } else {
            self.view_questionnaire(ctx)
        };

        html! {
            <main class="page">
                <section class="panel">
                    <header class="panel-header">
                        <h1>{"エニアグラム診断"}</h1>
                    </header>
                    {content}
                </section>
            </main>
        }
    }
}

impl App {
    fn active_result(&self) -> Option<DiagnosisResult> {
        if self.show_stored_result {
            return self.stored_result.as_ref().map(|saved| DiagnosisResult {
                scores: saved.scores,
                top_types: saved.top_types.clone(),
            });
        }

        self.current_result.clone()
    }

    fn view_questionnaire(&self, ctx: &Context<Self>) -> Html {
        let axis_total = self.questions.len();
        let axis_index = self.current_axis;
        let (completed_axes, progress_pct) = completed_axis_progress(&self.selections, axis_total);
        let axis = &self.questions[axis_index];
        let selection = &self.selections[axis_index];
        let is_last = axis_index + 1 == axis_total;

        let next_action = if is_last {
            ctx.link().callback(|_| Msg::Submit)
        } else {
            ctx.link().callback(|_| Msg::NextAxis)
        };

        let next_label = if is_last { "結果を見る" } else { "次へ" };

        html! {
            <>
                {
                    if let Some(saved) = &self.stored_result {
                        html! {
                            <div class="saved-box">
                                <div>
                                    <p>{"前回の診断結果があります"}</p>
                                    <small>{format!("保存日時: {}", saved.generated_at)}</small>
                                </div>
                                <button class="ghost-btn" onclick={ctx.link().callback(|_| Msg::ShowStoredResult)}>{"前回結果を表示"}</button>
                            </div>
                        }
                    } else {
                        html! {}
                    }
                }

                <div class="progress-wrap" role="progressbar" aria-valuenow={completed_axes.to_string()} aria-valuemin="0" aria-valuemax={axis_total.to_string()}>
                    <div class="progress-top">
                        <strong>{format!("進捗 {}/{}", completed_axes, axis_total)}</strong>
                    </div>
                    <div class="progress-track">
                        <div class="progress-fill" style={format!("width: {:.2}%;", progress_pct)}></div>
                    </div>
                </div>

                <div class="axis-heading">
                    <span>{axis.axis.label()}</span>
                </div>

                <div class="rank-guide">
                    <span class="chip rank-most">{"最も当てはまる: 1個"}</span>
                    <span class="chip rank-next">{"次に当てはまる: 1個"}</span>
                    <span class="chip rank-slight">{format!("少しは当てはまる: 2個  現在 {}/2", selection.slight.len())}</span>
                </div>

                <ul class="question-list">
                    {
                        for axis.items.iter().enumerate().map(|(idx, item)| {
                            let rank = selection.rank_of(idx);

                            let most_active = rank == Some(Rank::Most);
                            let next_active = rank == Some(Rank::Next);
                            let slight_active = rank == Some(Rank::Slight);

                            let slight_disabled = selection.slight.len() >= 2 && !slight_active;

                            let on_most = if most_active {
                                ctx.link().callback(move |_| Msg::ClearItem(idx))
                            } else {
                                ctx.link().callback(move |_| Msg::SelectRank { item_index: idx, rank: Rank::Most })
                            };

                            let on_next = if next_active {
                                ctx.link().callback(move |_| Msg::ClearItem(idx))
                            } else {
                                ctx.link().callback(move |_| Msg::SelectRank { item_index: idx, rank: Rank::Next })
                            };

                            let on_slight = if slight_active {
                                ctx.link().callback(move |_| Msg::ClearItem(idx))
                            } else {
                                ctx.link().callback(move |_| Msg::SelectRank { item_index: idx, rank: Rank::Slight })
                            };

                            html! {
                                <li class="question-card">
                                    <div class="question-meta">
                                        <p class="type-label">{format!("選択肢 {}", idx + 1)}</p>
                                    </div>
                                    <p class="question-text">{&item.text}</p>
                                    <div class="rank-buttons">
                                        <button class={classes!("rank-btn", "rank-most", most_active.then_some("is-active"))} onclick={on_most}>{Rank::Most.label()}</button>
                                        <button class={classes!("rank-btn", "rank-next", next_active.then_some("is-active"))} onclick={on_next}>{Rank::Next.label()}</button>
                                        <button
                                            class={classes!("rank-btn", "rank-slight", slight_active.then_some("is-active"))}
                                            onclick={on_slight}
                                            disabled={slight_disabled}
                                        >
                                            {Rank::Slight.label()}
                                        </button>
                                    </div>
                                </li>
                            }
                        })
                    }
                </ul>

                {
                    if let Some(message) = &self.message {
                        html! { <p class="form-message">{message}</p> }
                    } else {
                        html! {}
                    }
                }

                <div class={classes!("nav-buttons", (axis_index == 0).then_some("is-first"))}>
                    {
                        if axis_index > 0 {
                            html! {
                                <button class="ghost-btn" onclick={ctx.link().callback(|_| Msg::PrevAxis)}>{"戻る"}</button>
                            }
                        } else {
                            html! {}
                        }
                    }
                    <button class="primary-btn" onclick={next_action} disabled={!selection.is_complete()}>{next_label}</button>
                </div>
            </>
        }
    }

    fn view_result(&self, ctx: &Context<Self>, result: &DiagnosisResult) -> Html {
        let max_score = result.scores.iter().copied().max().unwrap_or(1).max(1);
        let top_labels = result
            .top_types
            .iter()
            .map(|t| t.label())
            .collect::<Vec<_>>()
            .join(" / ");

        html! {
            <div class="result-wrap">
                <h2>{"診断結果"}</h2>
                <p class="result-main">{format!("候補タイプ: {}", top_labels)}</p>

                <div class="profiles">
                    {
                        for result.top_types.iter().map(|t| {
                            html! {
                                <article class="profile-card">
                                    <h3>{t.label()}</h3>
                                    <p>{t.profile_placeholder()}</p>
                                </article>
                            }
                        })
                    }
                </div>

                <h3>{"タイプ別スコア"}</h3>
                <div class="bar-chart" aria-label="タイプ別スコアの横棒グラフ">
                    {
                        for EnneagramType::ALL.iter().map(|t| {
                            let score = result.scores[t.index()];
                            let width = (score as f64 / max_score as f64) * 100.0;

                            html! {
                                <div class="bar-row">
                                    <span class="bar-label">{t.label()}</span>
                                    <div class="bar-track">
                                        <div class="bar-fill" style={format!("width: {:.2}%;", width)}></div>
                                    </div>
                                    <span class="bar-value">{score}</span>
                                </div>
                            }
                        })
                    }
                </div>

                <p class="notice">
                    {"この診断表によるタイプ判別の精度には限界があり、参考情報として活用してください。より正確な判定には専門家によるカウンセリング等が推奨されます。"}
                </p>

                <div class="result-actions">
                    {
                        if self.show_stored_result {
                            html! {
                                <button class="ghost-btn" onclick={ctx.link().callback(|_| Msg::BackToForm)}>{"フォームに戻る"}</button>
                            }
                        } else {
                            html! {}
                        }
                    }
                    <button class="primary-btn" onclick={ctx.link().callback(|_| Msg::ResetForm)}>{"最初からやり直す"}</button>
                </div>
            </div>
        }
    }

    fn view_explanation(&self, ctx: &Context<Self>) -> Html {
        html! {
            <main class="page">
                <section class="panel">
                    <header class="panel-header">
                        <h1>{"エニアグラム診断"}</h1>
                    </header>
                    <div class="explanation-dialog">
                        <div class="explanation-content">
                            <h2>{"エニアグラムについて"}</h2>
                            <p>
                                {"エニアグラムとは、人間の性格を9つのタイプに分類し、それぞれの思考・行動パターンを明らかにする性格診断の手法です。"}
                                <br/>
                                {"7つの質問に、それぞれ9つの選択肢があります。"}
                                <br/>
                                {"その中で特にあなたの性格に当てはまるもの1位、2位、3位（2つ）の計4つを選んでください。"}
                            </p>
                            <button class="primary-btn" onclick={ctx.link().callback(|_| Msg::CloseExplanation)}>
                                {"診断を開始する"}
                            </button>
                        </div>
                    </div>
                </section>
            </main>
        }
    }
}

fn completed_axis_progress(selections: &[AxisSelection], total: usize) -> (usize, f64) {
    let completed_axes = selections
        .iter()
        .filter(|selection| selection.is_complete())
        .count();
    let progress_pct = if total == 0 {
        0.0
    } else {
        (completed_axes as f64 / total as f64) * 100.0
    };

    (completed_axes, progress_pct)
}

fn calculate_result(
    questions: &[crate::models::AxisQuestion],
    selections: &[AxisSelection],
) -> DiagnosisResult {
    let mut scores = [0_u32; 9];

    for (axis_question, axis_selection) in questions.iter().zip(selections.iter()) {
        for (item_idx, item) in axis_question.items.iter().enumerate() {
            let point = axis_selection.score_for_item(item_idx);
            scores[item.enneagram_type.index()] += point;
        }
    }

    let highest = scores.iter().copied().max().unwrap_or(0);
    let top_types = EnneagramType::ALL
        .iter()
        .copied()
        .filter(|t| scores[t.index()] == highest)
        .collect::<Vec<_>>();

    DiagnosisResult { scores, top_types }
}

#[cfg(test)]
mod tests {
    use super::completed_axis_progress;
    use crate::models::AxisSelection;

    #[test]
    fn progress_starts_empty_until_any_axis_is_completed() {
        let selections = vec![AxisSelection::default(); 7];

        let (completed_axes, progress_pct) = completed_axis_progress(&selections, 7);

        assert_eq!(completed_axes, 0);
        assert_eq!(progress_pct, 0.0);
    }
}

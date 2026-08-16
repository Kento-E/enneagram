use wasm_bindgen_futures::JsFuture;
use web_sys::window;
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
    shared_type: Option<EnneagramType>,
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
    CopyShareUrl,
    CloseExplanation,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let questions = build_questions();
        let selections = vec![AxisSelection::default(); questions.len()];
        let stored_result = load_result();
        let shared_type = shared_type_from_url();

        Self {
            questions,
            selections,
            current_axis: 0,
            current_result: None,
            stored_result,
            show_stored_result: shared_type.is_some(),
            shared_type,
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
                self.shared_type = None;
                true
            }
            Msg::CopyShareUrl => {
                if let Some(result) = self.active_result() {
                    let url = share_url(result.top_types[0]);
                    if let Some(browser_window) = window() {
                        let clipboard = browser_window.navigator().clipboard();
                        wasm_bindgen_futures::spawn_local(async move {
                            let _ = JsFuture::from(clipboard.write_text(&url)).await;
                        });
                        self.message = Some("共有URLをコピーしました。".to_string());
                    }
                }
                true
            }
            Msg::CloseExplanation => {
                self.show_explanation = false;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if self.show_explanation && self.shared_type.is_none() {
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
            if let Some(shared_type) = self.shared_type {
                return Some(DiagnosisResult {
                    scores: [0; 9],
                    top_types: vec![shared_type],
                });
            }
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
                <p class="result-main">{format!("優位なタイプ: {}", top_labels)}</p>

                <div class="profiles">
                    {
                        for result.top_types.iter().map(|t| {
                            let profile = t.profile();
                            html! {
                                <article class="profile-card">
                                    <div class="profile-heading">
                                        <div>
                                            <p class="profile-kicker">{"あなたの優位タイプ"}</p>
                                            <h3>{format!("{}（{}）", t.label(), profile.nickname)}</h3>
                                        </div>
                                        <span class="profile-number">{t.index() + 1}</span>
                                    </div>
                                    <div class="report-section">
                                        <h4>{"基本傾向"}</h4>
                                        <p>{profile.overview}</p>
                                    </div>
                                    <div class="report-grid">
                                        <div class="report-section">
                                            <h4>{"活かしやすい力"}</h4>
                                            <p>{profile.strengths}</p>
                                        </div>
                                        <div class="report-section">
                                            <h4>{"つまずきやすい点"}</h4>
                                            <p>{profile.challenges}</p>
                                        </div>
                                        <div class="report-section">
                                            <h4>{"対人関係の傾向"}</h4>
                                            <p>{profile.relationships}</p>
                                        </div>
                                        <div class="report-section">
                                            <h4>{"成長の視点"}</h4>
                                            <p>{profile.growth}</p>
                                        </div>
                                    </div>
                                </article>
                            }
                        })
                    }
                </div>

                {
                    if result.scores.iter().any(|score| *score > 0) {
                        html! {
                            <div class="share-box">
                                <label for="share-url">{"共有URL"}</label>
                                <div class="share-controls">
                                    <input id="share-url" value={share_url(result.top_types[0])} readonly=true />
                                    <button class="ghost-btn" onclick={ctx.link().callback(|_| Msg::CopyShareUrl)}>{"URLをコピー"}</button>
                                </div>
                            </div>
                        }
                    } else {
                        html! {}
                    }
                }

                {
                    if result.scores.iter().any(|score| *score > 0) {
                        html! {
                            <>
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
                            </>
                        }
                    } else {
                        html! {}
                    }
                }

                <p class="notice">
                    {"この診断表によるタイプ判別の精度には限界があり、参考情報として活用してください。より正確な判定には専門家によるカウンセリング等が推奨されます。"}
                </p>
                <p class="source-note">
                    {"タイプ名と特徴の整理は"}
                    <a href="https://www.enneagram.ne.jp/about/about_type" target="_blank" rel="noopener noreferrer">{"日本エニアグラム学会「各タイプの特徴」"}</a>
                    {"を参考にしています。"}
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

fn shared_type_from_url() -> Option<EnneagramType> {
    let search = window()?.location().search().ok()?;
    let value = search
        .trim_start_matches('?')
        .split('&')
        .find_map(|part| part.strip_prefix("type="))?
        .parse::<usize>()
        .ok()?;

    EnneagramType::ALL.get(value.checked_sub(1)?).copied()
}

fn share_url(enneagram_type: EnneagramType) -> String {
    let Some(browser_window) = window() else {
        return format!("?type={}", enneagram_type.index() + 1);
    };
    let Ok(origin) = browser_window.location().origin() else {
        return format!("?type={}", enneagram_type.index() + 1);
    };
    let Ok(pathname) = browser_window.location().pathname() else {
        return format!(
            "{}/?type={}",
            origin.trim_end_matches('/'),
            enneagram_type.index() + 1
        );
    };
    format!(
        "{}{}?type={}",
        origin.trim_end_matches('/'),
        pathname,
        enneagram_type.index() + 1
    )
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

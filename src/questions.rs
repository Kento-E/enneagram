use crate::models::{AxisId, AxisQuestion, EnneagramType, QuestionItem};

pub fn build_questions() -> Vec<AxisQuestion> {
    AxisId::ALL
        .iter()
        .enumerate()
        .map(|(axis_index, axis)| AxisQuestion {
            axis: *axis,
            items: EnneagramType::ALL
                .iter()
                .map(|t| QuestionItem {
                    enneagram_type: *t,
                    text: format!(
                        "[プレースホルダー] 分類軸{} / {} に対応する説明文",
                        axis_index + 1,
                        t.label()
                    ),
                })
                .collect(),
        })
        .collect()
}

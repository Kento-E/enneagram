use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnneagramType {
    Type1,
    Type2,
    Type3,
    Type4,
    Type5,
    Type6,
    Type7,
    Type8,
    Type9,
}

impl EnneagramType {
    pub const ALL: [Self; 9] = [
        Self::Type1,
        Self::Type2,
        Self::Type3,
        Self::Type4,
        Self::Type5,
        Self::Type6,
        Self::Type7,
        Self::Type8,
        Self::Type9,
    ];

    pub fn index(self) -> usize {
        match self {
            Self::Type1 => 0,
            Self::Type2 => 1,
            Self::Type3 => 2,
            Self::Type4 => 3,
            Self::Type5 => 4,
            Self::Type6 => 5,
            Self::Type7 => 6,
            Self::Type8 => 7,
            Self::Type9 => 8,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Type1 => "タイプ1",
            Self::Type2 => "タイプ2",
            Self::Type3 => "タイプ3",
            Self::Type4 => "タイプ4",
            Self::Type5 => "タイプ5",
            Self::Type6 => "タイプ6",
            Self::Type7 => "タイプ7",
            Self::Type8 => "タイプ8",
            Self::Type9 => "タイプ9",
        }
    }

    pub fn profile(self) -> TypeProfile {
        match self {
            Self::Type1 => TypeProfile { nickname: "改革する人", overview: "理想や基準を持ち、物事をより良い状態へ整えようとするタイプです。正しさ、公平さ、誠実さを重視し、自分自身にも高い水準を求めます。", strengths: "改善点を見抜く力、責任感、粘り強さ、筋道立てて行動する力。", challenges: "理想と現実の差に苛立ったり、できていない自分や周囲を厳しく評価しやすい傾向があります。", relationships: "率直で公正に接しようとしますが、助言や指摘が相手には批判として伝わることがあります。", growth: "完璧でない状態にも価値を認め、正しさだけでなく相手の事情や感情にも余白を持つことが助けになります。" },
            Self::Type2 => TypeProfile { nickname: "人を助ける人", overview: "人の困りごとや感情の変化に敏感で、役に立つことを通じてつながりを感じるタイプです。温かく世話好きで、相手の必要を先回りして支えようとします。", strengths: "共感力、気配り、親密な関係を育てる力、相手を励まして動かす力。", challenges: "助けた分だけ感謝や愛情を求め、思いが返ってこないと寂しさや怒りを抱えやすい傾向があります。", relationships: "親身で頼りになる一方、相手が望んでいない援助や過干渉にならないよう注意が必要です。", growth: "人を支える前に自分の望みや疲れを認め、見返りを条件にしない助け方と、率直な要望の伝達を練習するとよいでしょう。" },
            Self::Type3 => TypeProfile { nickname: "達成する人", overview: "明確な目標を定め、成果を出すために効率よく動くタイプです。周囲の期待や状況に合わせて自分を磨き、成功に向かう道筋を見つけるのが得意です。", strengths: "実行力、適応力、目標設定、周囲の才能を活かすリーダーシップ。", challenges: "失敗や評価の低下を避けるため、成果や見栄えを自分の価値と結びつけやすい傾向があります。", relationships: "明るく有能に見られ、チームを前進させますが、感情や弱さを置き去りにしてしまうことがあります。", growth: "達成したことだけでなく、感じていることや過程の自分にも価値があると認めることが、自然体の信頼につながります。" },
            Self::Type4 => TypeProfile { nickname: "個性的な人", overview: "独自の感性と内面の豊かさを大切にし、ありきたりではない自分らしい表現を求めるタイプです。人の気持ちや雰囲気の微細な違いにもよく気づきます。", strengths: "創造性、感受性、自己理解への探究心、深い共感と個性の表現。", challenges: "自分には欠けているものがあると感じたり、他人との比較や孤独感によって気分が揺れやすい傾向があります。", relationships: "表面的でない心のつながりを求め、理解し合える相手とは特別に深い関係を築きます。", growth: "強い感情を否定せずに受け止めながら、今ここにあるつながりや平凡な日常の価値にも目を向けると安定します。" },
            Self::Type5 => TypeProfile { nickname: "調べる人", overview: "まず観察し、考え、情報を集めてから行動するタイプです。自分の関心分野を深く掘り下げ、距離を保ちながら物事の本質を理解しようとします。", strengths: "分析力、専門知識、独立した思考、複雑なことを整理して洞察に変える力。", challenges: "自分の時間やエネルギーを守るために人や現実から距離を取りすぎ、知識の蓄積だけで止まりやすい傾向があります。", relationships: "静かで慎重に関わり、無理に踏み込まず相手を尊重しますが、周囲には冷淡に見えることがあります。", growth: "十分に準備できるまで待つのではなく、小さく共有し、小さく参加することで、知識を現実の関係や行動に活かせます。" },
            Self::Type6 => TypeProfile { nickname: "忠実な人", overview: "信頼できる人や仕組みとのつながりを大切にし、責任を果たしながら仲間を守ろうとするタイプです。危険や不確実さを先に見つけ、備えを整える力があります。", strengths: "誠実さ、責任感、リスクへの感度、協力して仕組みを守る力。", challenges: "間違いや裏切りへの不安から疑い深くなったり、決断の前に確認を重ねて動けなくなりやすい傾向があります。", relationships: "一度信頼した相手には忠実で温かく接しますが、権威や未知の相手には慎重になりやすいでしょう。", growth: "不安を消し切ってから動くのではなく、情報を確認した上で自分の判断も信頼し、段階的に決めることが力になります。" },
            Self::Type7 => TypeProfile { nickname: "熱中する人", overview: "人生の可能性を広げ、楽しい体験や新しい計画を次々に見つけるタイプです。明るく柔軟で、苦しい状況にも別の見方や面白さを持ち込めます。", strengths: "発想力、好奇心、楽観性、複数の可能性をつなげて場を活性化する力。", challenges: "苦痛や退屈を避けるために予定や興味を増やし、集中や仕上げ、深い感情への直面を後回しにしやすい傾向があります。", relationships: "気さくで人を楽しませ、自由を共有できる仲間を好みますが、深刻な話を軽く扱わない配慮も大切です。", growth: "選択肢を増やす前に一つの体験へ留まり、未完了のことや不快な感情にも少しずつ向き合うと、楽しさが深まります。" },
            Self::Type8 => TypeProfile { nickname: "挑戦する人", overview: "自分の力で状況を切り開き、正しいと思うことを率直に実行するタイプです。困難や対立を恐れず、弱い立場の人を守るためにも強さを使おうとします。", strengths: "決断力、行動力、率直さ、困難を引き受けて人を動かす推進力。", challenges: "弱さを見せたくない気持ちから、言葉や態度が強くなり、相手を圧倒したり対立を激しくしやすい傾向があります。", relationships: "本音で向き合うことを好み、頼られると力強く守りますが、相手の選択や速度を尊重する余地が必要です。", growth: "強さだけでなく傷つきやすさも関係の一部として示し、力を押し通す前に相手の声を聞くと、影響力がより建設的になります。" },
            Self::Type9 => TypeProfile { nickname: "平和をもたらす人", overview: "穏やかな内面と周囲の調和を大切にし、対立を和らげて人々をつなぐタイプです。自分から急に変化を起こすより、状況を受け止めながら自然な流れを保とうとします。", strengths: "受容力、調整力、落ち着き、異なる立場を同時に理解して場をまとめる力。", challenges: "波風を避けるために自分の希望や不満を後回しにし、決断や問題への着手を先送りしやすい傾向があります。", relationships: "誰に対しても穏やかで居心地のよい存在ですが、表面的な同意だけでは本当の意思が伝わりません。", growth: "小さな希望を言葉にし、優先順位を一つ決めて動き始めることで、自分の存在感と本来の行動力を取り戻せます。" },
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TypeProfile {
    pub nickname: &'static str,
    pub overview: &'static str,
    pub strengths: &'static str,
    pub challenges: &'static str,
    pub relationships: &'static str,
    pub growth: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AxisId {
    StrengthPreference,
    UnconsciousHabit,
    CommunicationStyle,
    RelationshipStyle,
    CommonPattern,
    AuthenticTrait,
    ExternalImpression,
}

impl AxisId {
    pub const ALL: [Self; 7] = [
        Self::StrengthPreference,
        Self::UnconsciousHabit,
        Self::CommunicationStyle,
        Self::RelationshipStyle,
        Self::CommonPattern,
        Self::AuthenticTrait,
        Self::ExternalImpression,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::StrengthPreference => "自分のこだわり(良いところ)",
            Self::UnconsciousHabit => "無意識のクセ(良くないところ)",
            Self::CommunicationStyle => "コミュニケーションの傾向",
            Self::RelationshipStyle => "対人関係の傾向",
            Self::CommonPattern => "よくあること",
            Self::AuthenticTrait => "自分らしいこと",
            Self::ExternalImpression => "他者から見える印象",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct QuestionItem {
    pub enneagram_type: EnneagramType,
    pub text: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AxisQuestion {
    pub axis: AxisId,
    pub items: Vec<QuestionItem>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rank {
    Most,
    Next,
    Slight,
}

impl Rank {
    pub fn label(self) -> &'static str {
        match self {
            Rank::Most => "最も当てはまる",
            Rank::Next => "次に当てはまる",
            Rank::Slight => "少しは当てはまる",
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct AxisSelection {
    pub most: Option<usize>,
    pub next: Option<usize>,
    pub slight: Vec<usize>,
}

impl AxisSelection {
    pub fn rank_of(&self, item_index: usize) -> Option<Rank> {
        if self.most == Some(item_index) {
            return Some(Rank::Most);
        }
        if self.next == Some(item_index) {
            return Some(Rank::Next);
        }
        if self.slight.contains(&item_index) {
            return Some(Rank::Slight);
        }
        None
    }

    pub fn is_complete(&self) -> bool {
        self.most.is_some() && self.next.is_some() && self.slight.len() == 2
    }

    pub fn score_for_item(&self, item_index: usize) -> u32 {
        if self.most == Some(item_index) {
            return 5;
        }
        if self.next == Some(item_index) {
            return 4;
        }
        if self.slight.contains(&item_index) {
            return 2;
        }
        0
    }

    pub fn assign_rank(&mut self, rank: Rank, item_index: usize) -> Result<(), &'static str> {
        self.clear_item(item_index);

        match rank {
            Rank::Most => {
                self.most = Some(item_index);
            }
            Rank::Next => {
                self.next = Some(item_index);
            }
            Rank::Slight => {
                if self.slight.len() >= 2 {
                    return Err("「少しは当てはまる」は2個までです。");
                }
                self.slight.push(item_index);
            }
        }

        Ok(())
    }

    pub fn clear_item(&mut self, item_index: usize) {
        if self.most == Some(item_index) {
            self.most = None;
        }
        if self.next == Some(item_index) {
            self.next = None;
        }
        self.slight.retain(|v| *v != item_index);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DiagnosisResult {
    pub scores: [u32; 9],
    pub top_types: Vec<EnneagramType>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StoredResult {
    pub scores: [u32; 9],
    pub top_types: Vec<EnneagramType>,
    pub generated_at: String,
}

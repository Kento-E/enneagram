use enneagram::models::{AxisSelection, EnneagramType, Rank};

#[test]
fn assigns_points_based_on_rank_selection() {
    let mut selection = AxisSelection::default();

    assert_assign_ok(&mut selection, Rank::Most, 1);
    assert_assign_ok(&mut selection, Rank::Next, 2);
    assert_assign_ok(&mut selection, Rank::Slight, 3);
    assert_assign_ok(&mut selection, Rank::Slight, 4);

    assert_eq!(selection.score_for_item(1), 5);
    assert_eq!(selection.score_for_item(2), 4);
    assert_eq!(selection.score_for_item(3), 2);
    assert_eq!(selection.score_for_item(4), 2);
    assert_eq!(selection.score_for_item(5), 0);
}

#[test]
fn rejects_third_slight_selection() {
    let mut selection = AxisSelection::default();

    assert_assign_ok(&mut selection, Rank::Slight, 1);
    assert_assign_ok(&mut selection, Rank::Slight, 2);

    let result = selection.assign_rank(Rank::Slight, 3);
    assert!(result.is_err());
}

#[test]
fn replacing_rank_for_same_item_keeps_single_active_rank() {
    let mut selection = AxisSelection::default();

    assert_assign_ok(&mut selection, Rank::Most, 2);
    assert_assign_ok(&mut selection, Rank::Next, 2);

    assert_eq!(selection.rank_of(2), Some(Rank::Next));
    assert_eq!(selection.score_for_item(2), 4);
}

#[test]
fn reports_incomplete_until_most_next_and_two_slight_are_selected() {
    let mut selection = AxisSelection::default();

    assert!(!selection.is_complete());
    assert_assign_ok(&mut selection, Rank::Most, 0);
    assert_assign_ok(&mut selection, Rank::Next, 1);
    assert!(!selection.is_complete());

    assert_assign_ok(&mut selection, Rank::Slight, 2);
    assert!(!selection.is_complete());
    assert_assign_ok(&mut selection, Rank::Slight, 3);

    assert!(selection.is_complete());
    assert_eq!(selection.rank_of(0), Some(Rank::Most));
    assert_eq!(selection.rank_of(1), Some(Rank::Next));
    assert_eq!(selection.rank_of(2), Some(Rank::Slight));
    assert_eq!(selection.rank_of(3), Some(Rank::Slight));
}

#[test]
fn enneagram_type_indexes_match_all_order() {
    for (expected_index, enneagram_type) in EnneagramType::ALL.iter().copied().enumerate() {
        assert_eq!(enneagram_type.index(), expected_index);
    }
}

fn assert_assign_ok(selection: &mut AxisSelection, rank: Rank, item_index: usize) {
    let result = selection.assign_rank(rank, item_index);
    assert_eq!(result, Ok(()));
}

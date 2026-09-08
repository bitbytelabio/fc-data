#![doc = "Typed channel selector contracts."]

use ssi_fc_data::stream::{Channel, ChannelError, ChannelSelector};

#[test]
fn renders_multi_symbol_selector() {
    let selector = ChannelSelector::symbols(["SSI", "PAN"]).expect("valid symbols");
    assert_eq!(Channel::quote(&selector).to_string(), "X-QUOTE:SSI-PAN");
}

#[test]
fn trims_whitespace_from_symbols() {
    let selector =
        ChannelSelector::symbols(["  SSI  ", "\tPAN\n"]).expect("valid symbols with whitespace");
    assert_eq!(Channel::trade(&selector).to_string(), "X-TRADE:SSI-PAN");
}

#[test]
fn preserves_index_case_without_alteration() {
    let selector = ChannelSelector::symbols(["VN30", "HNXindex"]).expect("valid indexes");
    assert_eq!(Channel::index(&selector).to_string(), "MI:VN30-HNXindex");
}

#[test]
fn rejects_empty_symbol_list() {
    let err = ChannelSelector::symbols(std::iter::empty::<&str>()).expect_err("empty should fail");
    assert!(matches!(err, ChannelError::Empty));
}

#[test]
fn rejects_empty_or_whitespace_subject() {
    let err = ChannelSelector::symbols(["SSI", "   "]).expect_err("whitespace subject should fail");
    assert!(matches!(err, ChannelError::Empty));

    let err_single = ChannelSelector::symbols([""]).expect_err("empty subject should fail");
    assert!(matches!(err_single, ChannelError::Empty));
}

#[test]
fn rejects_all_within_symbols_case_insensitively() {
    let err_upper = ChannelSelector::symbols(["ALL", "SSI"]).expect_err("ALL should fail");
    assert!(matches!(err_upper, ChannelError::AllMisuse));

    let err_lower = ChannelSelector::symbols(["all"]).expect_err("lowercase all should fail");
    assert!(matches!(err_lower, ChannelError::AllMisuse));

    let err_mixed =
        ChannelSelector::symbols(["SSI", "  All  "]).expect_err("mixed case All should fail");
    assert!(matches!(err_mixed, ChannelError::AllMisuse));
}

#[test]
fn raw_channel_remains_permissive_except_empty() {
    let channel = Channel::raw("CUSTOM:ANYTHING-123").expect("permissive custom channel");
    assert_eq!(channel.as_str(), "CUSTOM:ANYTHING-123");

    let empty_err = Channel::raw("   ").expect_err("empty raw channel should fail");
    assert!(matches!(empty_err, ChannelError::Empty));
}

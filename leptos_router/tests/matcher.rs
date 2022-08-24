// Test cases drawn from Solid Router
// see https://github.com/solidjs/solid-router/blob/main/test/utils.spec.ts

use leptos_router::{params_map, Matcher, PathMatch};

#[test]
fn create_matcher_should_return_no_params_when_location_matches_exactly() {
    let matcher = Matcher::new("/foo/bar");
    let matched = matcher.test("/foo/bar");
    assert_eq!(
        matched,
        Some(PathMatch {
            path: "/foo/bar".into(),
            params: params_map!()
        })
    );
}

#[test]
fn create_matcher_should_return_none_when_location_doesnt_match() {
    let matcher = Matcher::new("/foo/bar");
    let matched = matcher.test("/foo/baz");
    assert_eq!(matched, None);
}

#[test]
fn create_matcher_should_build_params_collection() {
    let matcher = Matcher::new("/foo/:id");
    let matched = matcher.test("/foo/abc-123");
    assert_eq!(
        matched,
        Some(PathMatch {
            path: "/foo/abc-123".into(),
            params: params_map!(
                "id".into() => "abc-123".into()
            )
        })
    );
}

#[test]
fn create_matcher_should_match_past_end_when_ending_in_asterisk() {
    let matcher = Matcher::new("/foo/bar/*");
    let matched = matcher.test("/foo/bar/baz");
    assert_eq!(
        matched,
        Some(PathMatch {
            path: "/foo/bar".into(),
            params: params_map!()
        })
    );
}

#[test]
fn create_matcher_should_not_match_past_end_when_not_ending_in_asterisk() {
    let matcher = Matcher::new("/foo/bar");
    let matched = matcher.test("/foo/bar/baz");
    assert_eq!(matched, None);
}

#[test]
fn create_matcher_should_include_remaining_unmatched_location_as_param_when_ending_in_asterisk_and_name(
) {
    let matcher = Matcher::new("/foo/bar/*something");
    let matched = matcher.test("/foo/bar/baz/qux");
    assert_eq!(
        matched,
        Some(PathMatch {
            path: "/foo/bar".into(),
            params: params_map!(
                "something".into() => "baz/qux".into()
            )
        })
    );
}

#[test]
fn create_matcher_should_include_empty_param_when_perfect_match_ends_in_asterisk_and_name() {
    let matcher = Matcher::new("/foo/bar/*something");
    let matched = matcher.test("/foo/bar");
    assert_eq!(
        matched,
        Some(PathMatch {
            path: "/foo/bar".into(),
            params: params_map!(
                "something".into() => "".into()
            )
        })
    );
}

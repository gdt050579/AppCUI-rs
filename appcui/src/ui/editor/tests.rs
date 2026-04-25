use super::chunk::Chunk;

// ============================================================================
// Chunk tests
// ============================================================================

fn assert_chunk_state(chunk: &Chunk, expected_bytes: &[u8]) {
    assert_eq!(chunk.bytes(), expected_bytes, "byte content");
    let expected_lines = expected_bytes.iter().filter(|&&b| b == b'\n').count() as u16;
    assert_eq!(chunk.line_count(), expected_lines, "line_count");
    assert!(chunk.check_invariants(), "invariants violated");
}

#[test]
fn chunk_new_is_empty() {
    let chunk = Chunk::new();
    assert_eq!(chunk.len(), 0);
    assert!(chunk.is_empty());
    assert!(chunk.is_ascii());
    assert!(!chunk.has_tabs());
    assert_chunk_state(&chunk, b"");
}

#[test]
fn chunk_from_bytes_empty() {
    let chunk = Chunk::from_bytes(vec![]);
    assert_chunk_state(&chunk, b"");
}

#[test]
fn chunk_from_bytes_no_newlines() {
    let chunk = Chunk::from_bytes(b"hello".to_vec());
    assert_chunk_state(&chunk, b"hello");
}

#[test]
fn chunk_from_bytes_trailing_newline() {
    let chunk = Chunk::from_bytes(b"hello\n".to_vec());
    assert_chunk_state(&chunk, b"hello\n");
    assert_eq!(chunk.line_range(1), 6..6);
}

#[test]
fn chunk_from_bytes_multiple_lines() {
    let chunk = Chunk::from_bytes(b"a\nb\nc".to_vec());
    assert_chunk_state(&chunk, b"a\nb\nc");
    assert_eq!(chunk.line_range(0), 0..1);
    assert_eq!(chunk.line_range(1), 2..3);
    assert_eq!(chunk.line_range(2), 4..5);
}

#[test]
fn chunk_from_bytes_consecutive_newlines() {
    let chunk = Chunk::from_bytes(b"\n\n\n".to_vec());
    assert_chunk_state(&chunk, b"\n\n\n");
    assert_eq!(chunk.line_range(0), 0..0);
    assert_eq!(chunk.line_range(1), 1..1);
    assert_eq!(chunk.line_range(2), 2..2);
    assert_eq!(chunk.line_range(3), 3..3);
}

#[test]
fn chunk_from_bytes_ascii_detection() {
    let ascii = Chunk::from_slice(b"hello");
    let non_ascii = Chunk::from_slice("h\u{00E9}llo".as_bytes());
    assert!(ascii.is_ascii());
    assert!(!non_ascii.is_ascii());
}

#[test]
fn chunk_from_bytes_tab_detection() {
    let no_tabs = Chunk::from_slice(b"hello");
    let tabs = Chunk::from_slice(b"he\tllo");
    assert!(!no_tabs.has_tabs());
    assert!(tabs.has_tabs());
}

#[test]
fn chunk_len_empty_and_nonempty() {
    assert_eq!(Chunk::new().len(), 0);
    assert_eq!(Chunk::from_slice(b"abc").len(), 3);
}

#[test]
fn chunk_byte_at_returns_correct_byte() {
    let chunk = Chunk::from_slice(b"abc");
    assert_eq!(chunk.byte_at(0), b'a');
    assert_eq!(chunk.byte_at(2), b'c');
}

#[test]
#[should_panic]
fn chunk_byte_at_out_of_bounds_panics() {
    let chunk = Chunk::from_slice(b"abc");
    let _ = chunk.byte_at(3);
}

#[test]
fn chunk_slice_returns_correct_bytes() {
    let chunk = Chunk::from_slice(b"abcdef");
    assert_eq!(chunk.slice(1..4), b"bcd");
}

#[test]
fn chunk_line_range_first_line() {
    let chunk = Chunk::from_slice(b"abc\ndef");
    assert_eq!(chunk.line_range(0), 0..3);
}

#[test]
fn chunk_line_range_last_line_no_newline() {
    let chunk = Chunk::from_slice(b"abc\ndef");
    assert_eq!(chunk.line_range(1), 4..7);
}

#[test]
fn chunk_line_range_last_line_with_newline() {
    let chunk = Chunk::from_slice(b"abc\n");
    assert_eq!(chunk.line_range(1), 4..4);
}

#[test]
fn chunk_line_range_empty_line() {
    let chunk = Chunk::from_slice(b"a\n\nb");
    assert_eq!(chunk.line_range(1), 2..2);
}

#[test]
fn chunk_line_at_offset_at_line_start() {
    let chunk = Chunk::from_slice(b"a\nbc\n");
    assert_eq!(chunk.line_at_offset(2), 1);
}

#[test]
fn chunk_line_at_offset_at_newline() {
    let chunk = Chunk::from_slice(b"a\nbc");
    assert_eq!(chunk.line_at_offset(1), 0);
}

#[test]
fn chunk_line_at_offset_at_end() {
    let chunk = Chunk::from_slice(b"a\nbc");
    assert_eq!(chunk.line_at_offset(chunk.len() as u16), 1);
}

#[test]
fn chunk_insert_empty_slice_is_noop() {
    let mut chunk = Chunk::from_slice(b"abc");
    chunk.insert(1, b"");
    assert_chunk_state(&chunk, b"abc");
}

#[test]
fn chunk_insert_at_front() {
    let mut chunk = Chunk::from_slice(b"bc");
    chunk.insert(0, b"a");
    assert_chunk_state(&chunk, b"abc");
}

#[test]
fn chunk_insert_at_end() {
    let mut chunk = Chunk::from_slice(b"ab");
    chunk.insert(2, b"c");
    assert_chunk_state(&chunk, b"abc");
}

#[test]
fn chunk_insert_in_middle() {
    let mut chunk = Chunk::from_slice(b"ac");
    chunk.insert(1, b"b");
    assert_chunk_state(&chunk, b"abc");
}

#[test]
fn chunk_insert_into_empty_chunk() {
    let mut chunk = Chunk::new();
    chunk.insert(0, b"abc");
    assert_chunk_state(&chunk, b"abc");
}

#[test]
fn chunk_insert_at_line_boundary() {
    let mut chunk = Chunk::from_slice(b"ab\ncd");
    chunk.insert(3, b"X");
    assert_chunk_state(&chunk, b"ab\nXcd");
}

#[test]
fn chunk_insert_splits_existing_line() {
    let mut chunk = Chunk::from_slice(b"abcd");
    chunk.insert(2, b"\n");
    assert_chunk_state(&chunk, b"ab\ncd");
}

#[test]
fn chunk_insert_with_newlines_increases_line_count() {
    let mut chunk = Chunk::from_slice(b"ab");
    chunk.insert(1, b"\n\n");
    assert_chunk_state(&chunk, b"a\n\nb");
}

#[test]
fn chunk_insert_ascii_into_ascii_stays_ascii() {
    let mut chunk = Chunk::from_slice(b"abc");
    chunk.insert(1, b"XYZ");
    assert!(chunk.is_ascii());
}

#[test]
fn chunk_insert_non_ascii_clears_ascii_flag() {
    let mut chunk = Chunk::from_slice(b"abc");
    chunk.insert(1, "é".as_bytes());
    assert!(!chunk.is_ascii());
}

#[test]
fn chunk_insert_into_non_ascii_stays_non_ascii() {
    let mut chunk = Chunk::from_slice("é".as_bytes());
    chunk.insert(1, b"abc");
    assert!(!chunk.is_ascii());
}

#[test]
fn chunk_insert_tab_sets_tab_flag() {
    let mut chunk = Chunk::from_slice(b"abc");
    chunk.insert(1, b"\t");
    assert!(chunk.has_tabs());
}

#[test]
fn chunk_delete_empty_range_is_noop() {
    let mut chunk = Chunk::from_slice(b"abc");
    chunk.delete(1..1);
    assert_chunk_state(&chunk, b"abc");
}

#[test]
fn chunk_delete_single_byte() {
    let mut chunk = Chunk::from_slice(b"abc");
    chunk.delete(1..2);
    assert_chunk_state(&chunk, b"ac");
}

#[test]
fn chunk_delete_entire_contents() {
    let mut chunk = Chunk::from_slice(b"abc");
    chunk.delete(0..3);
    assert_chunk_state(&chunk, b"");
}

#[test]
fn chunk_delete_from_front() {
    let mut chunk = Chunk::from_slice(b"abc");
    chunk.delete(0..1);
    assert_chunk_state(&chunk, b"bc");
}

#[test]
fn chunk_delete_from_end() {
    let mut chunk = Chunk::from_slice(b"abc");
    chunk.delete(2..3);
    assert_chunk_state(&chunk, b"ab");
}

#[test]
fn chunk_delete_from_middle() {
    let mut chunk = Chunk::from_slice(b"abcde");
    chunk.delete(1..4);
    assert_chunk_state(&chunk, b"ae");
}

#[test]
fn chunk_delete_one_newline() {
    let mut chunk = Chunk::from_slice(b"a\nb");
    chunk.delete(1..2);
    assert_chunk_state(&chunk, b"ab");
}

#[test]
fn chunk_delete_multiple_newlines() {
    let mut chunk = Chunk::from_slice(b"a\nb\nc\n");
    chunk.delete(1..6);
    assert_chunk_state(&chunk, b"a");
}

#[test]
fn chunk_delete_starting_at_line_boundary() {
    let mut chunk = Chunk::from_slice(b"aaaa\nbbbb\ncccc");
    chunk.delete(5..10);
    assert_chunk_state(&chunk, b"aaaa\ncccc");
    assert_eq!(chunk.line_range(0), 0..4);
    assert_eq!(chunk.line_range(1), 5..9);
}

#[test]
fn chunk_delete_ending_at_line_boundary() {
    let mut chunk = Chunk::from_slice(b"aaaa\nbbbb\ncccc");
    chunk.delete(3..10);
    assert_chunk_state(&chunk, b"aaacccc");
}

#[test]
fn chunk_delete_spanning_multiple_lines() {
    let mut chunk = Chunk::from_slice(b"a\nb\nc\nd");
    chunk.delete(1..6);
    assert_chunk_state(&chunk, b"ad");
}

#[test]
fn chunk_delete_only_newline() {
    let mut chunk = Chunk::from_slice(b"ab\ncd");
    chunk.delete(2..3);
    assert_chunk_state(&chunk, b"abcd");
}

#[test]
fn chunk_delete_does_not_refresh_ascii_flag() {
    let mut chunk = Chunk::from_slice(b"abc");
    chunk.insert(1, "é".as_bytes());
    assert!(!chunk.is_ascii());
    chunk.delete(1..3);
    assert!(!chunk.is_ascii());
}

#[test]
fn chunk_append_empty_to_nonempty() {
    let mut left = Chunk::from_slice(b"abc");
    left.append(Chunk::new());
    assert_chunk_state(&left, b"abc");
}

#[test]
fn chunk_append_nonempty_to_empty() {
    let mut left = Chunk::new();
    left.append(Chunk::from_slice(b"abc"));
    assert_chunk_state(&left, b"abc");
}

#[test]
fn chunk_append_two_chunks_no_newlines() {
    let mut left = Chunk::from_slice(b"ab");
    left.append(Chunk::from_slice(b"cd"));
    assert_chunk_state(&left, b"abcd");
}

#[test]
fn chunk_append_left_ends_with_newline() {
    let mut left = Chunk::from_slice(b"ab\n");
    left.append(Chunk::from_slice(b"cd"));
    assert_chunk_state(&left, b"ab\ncd");
}

#[test]
fn chunk_append_right_starts_with_newline() {
    let mut left = Chunk::from_slice(b"ab");
    left.append(Chunk::from_slice(b"\ncd"));
    assert_chunk_state(&left, b"ab\ncd");
}

#[test]
fn chunk_append_combines_flags_correctly() {
    let mut left = Chunk::from_slice(b"abc");
    left.append(Chunk::from_slice("é\t".as_bytes()));
    assert!(!left.is_ascii());
    assert!(left.has_tabs());
}

#[test]
fn chunk_append_line_count_correct() {
    let mut left = Chunk::from_slice(b"a\nb");
    let right = Chunk::from_slice(b"\nc");
    left.append(right);
    let expected = Chunk::from_slice(b"a\nb\nc");
    assert_eq!(left.line_count(), expected.line_count());
    assert_eq!(left.bytes(), expected.bytes());
}

#[test]
fn chunk_split_at_zero_left_is_empty() {
    let mut left = Chunk::from_slice(b"abc");
    let right = left.split_off(0);
    assert_chunk_state(&left, b"");
    assert_chunk_state(&right, b"abc");
}

#[test]
fn chunk_split_at_end_right_is_empty() {
    let mut left = Chunk::from_slice(b"abc");
    let right = left.split_off(3);
    assert_chunk_state(&left, b"abc");
    assert_chunk_state(&right, b"");
}

#[test]
fn chunk_split_in_middle() {
    let mut left = Chunk::from_slice(b"abcdef");
    let right = left.split_off(3);
    assert_chunk_state(&left, b"abc");
    assert_chunk_state(&right, b"def");
}

#[test]
fn chunk_split_at_line_boundary() {
    let mut left = Chunk::from_slice(b"ab\ncd");
    let right = left.split_off(3);
    assert_chunk_state(&left, b"ab\n");
    assert_chunk_state(&right, b"cd");
}

#[test]
fn chunk_split_after_newline() {
    let mut left = Chunk::from_slice(b"a\nbc");
    let right = left.split_off(2);
    assert_chunk_state(&left, b"a\n");
    assert_chunk_state(&right, b"bc");
}

#[test]
fn chunk_split_no_newlines() {
    let mut left = Chunk::from_slice(b"abcd");
    let right = left.split_off(1);
    assert_chunk_state(&left, b"a");
    assert_chunk_state(&right, b"bcd");
}

#[test]
fn chunk_split_ascii_chunk_both_halves_ascii() {
    let mut left = Chunk::from_slice(b"abcdef");
    let right = left.split_off(3);
    assert!(left.is_ascii());
    assert!(right.is_ascii());
}

#[test]
fn chunk_split_mixed_chunk_flags_per_half() {
    let mut left = Chunk::from_slice("é\tabc".as_bytes());
    let right = left.split_off(3);
    assert!(!left.is_ascii());
    assert!(left.has_tabs());
    assert!(right.is_ascii());
    assert!(!right.has_tabs());
}

#[test]
fn chunk_split_then_append_round_trip() {
    let mut left = Chunk::from_slice(b"a\nb\tc");
    let right = left.split_off(2);
    left.append(right);
    assert_chunk_state(&left, b"a\nb\tc");
}

#[test]
fn chunk_insert_then_delete_round_trip() {
    let mut chunk = Chunk::from_slice(b"abcdef");
    chunk.insert(3, b"\n\t");
    chunk.delete(3..5);
    assert_chunk_state(&chunk, b"abcdef");
}

#[test]
fn chunk_split_then_append_recovers_original() {
    let original = b"ab\ncd\nef";
    let mut chunk = Chunk::from_slice(original);
    let right = chunk.split_off(4);
    chunk.append(right);
    assert_chunk_state(&chunk, original);
}

#[test]
fn chunk_refresh_flags_recomputes_exact_values() {
    let mut chunk = Chunk::from_slice(b"a\t");
    chunk.delete(1..2);
    assert!(chunk.has_tabs());
    chunk.refresh_flags();
    assert!(!chunk.has_tabs());
}

#[test]
fn chunk_fuzz_against_reference_vec() {
    let mut state: u64 = 0xDEADBEEF;
    let mut rng = || {
        state = state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        state
    };

    let mut chunk = Chunk::new();
    let mut reference: Vec<u8> = Vec::new();

    for _ in 0..2000 {
        match rng() % 3 {
            0 => {
                let offset = if reference.is_empty() {
                    0
                } else {
                    (rng() as usize) % (reference.len() + 1)
                };
                let len = (rng() as usize) % 8 + 1;
                let bytes: Vec<u8> = (0..len)
                    .map(|_| {
                        let r = rng() % 10;
                        if r == 0 {
                            b'\n'
                        } else if r == 1 {
                            b'\t'
                        } else if r == 2 {
                            0xC3
                        } else if r == 3 {
                            0xA9
                        } else {
                            b'a' + (rng() % 26) as u8
                        }
                    })
                    .collect();
                chunk.insert(offset as u16, &bytes);
                reference.splice(offset..offset, bytes);
            }
            1 => {
                if reference.is_empty() {
                    continue;
                }
                let start = (rng() as usize) % reference.len();
                let end = start + (rng() as usize) % (reference.len() - start + 1);
                chunk.delete(start as u16..end as u16);
                reference.drain(start..end);
            }
            _ => {}
        }

        assert_eq!(chunk.bytes(), reference.as_slice());
        assert!(chunk.check_invariants());
    }
}

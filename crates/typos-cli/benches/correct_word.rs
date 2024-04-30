#![allow(elided_lifetimes_in_paths)]

mod regular {
    mod ok {
        #[divan::bench]
        fn en(bencher: divan::Bencher) {
            let input = "finalizes";
            let locale = typos_cli::config::Locale::En;
            let corrections = typos_cli::dict::BuiltIn::new(locale);
            let input = typos::tokens::Word::new(input, 0).unwrap();
            #[cfg(feature = "vars")]
            assert_eq!(corrections.correct_word(input), None);
            bencher
                .with_inputs(|| input)
                .bench_local_values(|input| corrections.correct_word(input));
        }

        #[divan::bench]
        #[cfg(feature = "vars")]
        fn en_us(bencher: divan::Bencher) {
            let input = "finalizes";
            let locale = typos_cli::config::Locale::EnUs;
            let corrections = typos_cli::dict::BuiltIn::new(locale);
            let input = typos::tokens::Word::new(input, 0).unwrap();
            #[cfg(feature = "vars")]
            assert_eq!(corrections.correct_word(input), Some(typos::Status::Valid));
            bencher
                .with_inputs(|| input)
                .bench_local_values(|input| corrections.correct_word(input));
        }
    }

    mod misspell {
        #[divan::bench]
        fn en(bencher: divan::Bencher) {
            let input = "finallizes";
            let output = "finalizes";
            let locale = typos_cli::config::Locale::En;
            let corrections = typos_cli::dict::BuiltIn::new(locale);
            let input = typos::tokens::Word::new(input, 0).unwrap();
            assert_eq!(
                corrections.correct_word(input),
                Some(typos::Status::Corrections(vec![
                    std::borrow::Cow::Borrowed(output)
                ]))
            );
            bencher
                .with_inputs(|| input)
                .bench_local_values(|input| corrections.correct_word(input));
        }

        #[divan::bench]
        #[cfg(feature = "vars")]
        fn en_us(bencher: divan::Bencher) {
            let input = "finallizes";
            let output = "finalizes";
            let locale = typos_cli::config::Locale::EnUs;
            let corrections = typos_cli::dict::BuiltIn::new(locale);
            let input = typos::tokens::Word::new(input, 0).unwrap();
            assert_eq!(
                corrections.correct_word(input),
                Some(typos::Status::Corrections(vec![
                    std::borrow::Cow::Borrowed(output)
                ]))
            );
            bencher
                .with_inputs(|| input)
                .bench_local_values(|input| corrections.correct_word(input));
        }
    }

    mod misspell_case {
        #[divan::bench]
        fn en(bencher: divan::Bencher) {
            let input = "FINALLIZES";
            let output = "FINALIZES";
            let locale = typos_cli::config::Locale::En;
            let corrections = typos_cli::dict::BuiltIn::new(locale);
            let input = typos::tokens::Word::new(input, 0).unwrap();
            assert_eq!(
                corrections.correct_word(input),
                Some(typos::Status::Corrections(vec![
                    std::borrow::Cow::Borrowed(output)
                ]))
            );
            bencher
                .with_inputs(|| input)
                .bench_local_values(|input| corrections.correct_word(input));
        }

        #[divan::bench]
        #[cfg(feature = "vars")]
        fn en_us(bencher: divan::Bencher) {
            let input = "FINALLIZES";
            let output = "FINALIZES";
            let locale = typos_cli::config::Locale::EnUs;
            let corrections = typos_cli::dict::BuiltIn::new(locale);
            let input = typos::tokens::Word::new(input, 0).unwrap();
            assert_eq!(
                corrections.correct_word(input),
                Some(typos::Status::Corrections(vec![
                    std::borrow::Cow::Borrowed(output)
                ]))
            );
            bencher
                .with_inputs(|| input)
                .bench_local_values(|input| corrections.correct_word(input));
        }
    }
}

#[cfg(feature = "vars")]
mod varcon {
    mod ok {
        #[divan::bench]
        fn en_gb(bencher: divan::Bencher) {
            let input = "finalizes";
            let output = "finalises";
            let locale = typos_cli::config::Locale::EnGb;
            let corrections = typos_cli::dict::BuiltIn::new(locale);
            let input = typos::tokens::Word::new(input, 0).unwrap();
            assert_eq!(
                corrections.correct_word(input),
                Some(typos::Status::Corrections(vec![
                    std::borrow::Cow::Borrowed(output)
                ]))
            );
            bencher
                .with_inputs(|| input)
                .bench_local_values(|input| corrections.correct_word(input));
        }
    }

    mod misspell {
        #[divan::bench]
        fn en_gb(bencher: divan::Bencher) {
            let input = "finallizes";
            let output = "finalises";
            let locale = typos_cli::config::Locale::EnGb;
            let corrections = typos_cli::dict::BuiltIn::new(locale);
            let input = typos::tokens::Word::new(input, 0).unwrap();
            assert_eq!(
                corrections.correct_word(input),
                Some(typos::Status::Corrections(vec![
                    std::borrow::Cow::Borrowed(output)
                ]))
            );
            bencher
                .with_inputs(|| input)
                .bench_local_values(|input| corrections.correct_word(input));
        }
    }
}

fn main() {
    divan::main();
}

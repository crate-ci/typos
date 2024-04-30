#![allow(elided_lifetimes_in_paths)]

mod data;

mod parse_str {
    use super::data;

    #[divan::bench(args = data::DATA)]
    fn ascii(bencher: divan::Bencher, sample: &data::Data) {
        let unicode = false;
        let parser = typos::tokens::TokenizerBuilder::new()
            .unicode(unicode)
            .build();
        bencher
            .with_inputs(|| sample.content())
            .input_counter(divan::counter::BytesCount::of_str)
            .bench_local_values(|sample| parser.parse_str(sample).last());
    }

    #[divan::bench(args = data::DATA)]
    fn unicode(bencher: divan::Bencher, sample: &data::Data) {
        let unicode = true;
        let parser = typos::tokens::TokenizerBuilder::new()
            .unicode(unicode)
            .build();
        bencher
            .with_inputs(|| sample.content())
            .input_counter(divan::counter::BytesCount::of_str)
            .bench_local_values(|sample| parser.parse_str(sample).last());
    }
}

mod parse_bytes {
    use super::data;

    #[divan::bench(args = data::DATA)]
    fn ascii(bencher: divan::Bencher, sample: &data::Data) {
        let unicode = false;
        let parser = typos::tokens::TokenizerBuilder::new()
            .unicode(unicode)
            .build();
        bencher
            .with_inputs(|| sample.content().as_bytes())
            .input_counter(divan::counter::BytesCount::of_slice)
            .bench_local_values(|sample| parser.parse_bytes(sample).last());
    }

    #[divan::bench(args = data::DATA)]
    fn unicode(bencher: divan::Bencher, sample: &data::Data) {
        let unicode = true;
        let parser = typos::tokens::TokenizerBuilder::new()
            .unicode(unicode)
            .build();
        bencher
            .with_inputs(|| sample.content().as_bytes())
            .input_counter(divan::counter::BytesCount::of_slice)
            .bench_local_values(|sample| parser.parse_bytes(sample).last());
    }
}

#[divan::bench(args = data::DATA)]
fn split(bencher: divan::Bencher, sample: &data::Data) {
    let symbol =
        typos::tokens::Identifier::new_unchecked(sample.content(), typos::tokens::Case::None, 0);
    bencher
        .counter(divan::counter::BytesCount::of_str(sample.content()))
        .bench_local(|| symbol.split().last());
}

mod parse_split_bytes {
    use super::data;

    #[divan::bench(args = data::DATA)]
    fn ascii(bencher: divan::Bencher, sample: &data::Data) {
        let unicode = false;
        let parser = typos::tokens::TokenizerBuilder::new()
            .unicode(unicode)
            .build();
        bencher
            .with_inputs(|| sample.content().as_bytes())
            .input_counter(divan::counter::BytesCount::of_slice)
            .bench_local_values(|sample| parser.parse_bytes(sample).flat_map(|i| i.split()).last());
    }

    #[divan::bench(args = data::DATA)]
    fn unicode(bencher: divan::Bencher, sample: &data::Data) {
        let unicode = true;
        let parser = typos::tokens::TokenizerBuilder::new()
            .unicode(unicode)
            .build();
        bencher
            .with_inputs(|| sample.content().as_bytes())
            .input_counter(divan::counter::BytesCount::of_slice)
            .bench_local_values(|sample| parser.parse_bytes(sample).flat_map(|i| i.split()).last());
    }
}

fn main() {
    divan::main();
}

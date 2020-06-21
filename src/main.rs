fn main() {
    let yume = LyricBuilder::new()
        .aremo_shitai(true)
        .koremo_shitai(true)
        .motto_shitai(true)
        .mottomotto_shitai(true)
        .has_yume(true)
        .finalize();

    let seishunmode_mouikkai = LyricBuilder::new()
        .aremo_shitai(true)
        .koremo_shitai(true)
        .finalize();

    let get_along = LyricBuilder::new()
        .aremo_shitai(true)
        .koremo_shitai(true)
        .finalize();

    let mirai_heno_tegami = LyricBuilder::new()
        .aremo_shitai(true)
        .koremo_shitai(true)
        .has_yume(true)
        .finalize();

    let my_land = LyricBuilder::new()
        .aremo_shitai(true)
        .koremo_shitai(true)
        .has_yume(true)
        .finalize();

    let lyrics = vec![
        yume,
        seishunmode_mouikkai,
        get_along,
        mirai_heno_tegami,
        my_land,
    ];

    for lyric in lyrics {
        match lyric.is_yume() {
            true => println!("そう、それそれ"),
            false => println!("違う、そうじゃない"),
        }
    }
}

#[derive(Debug)]
struct Lyric {
    aremo_shitai: bool,
    koremo_shitai: bool,
    motto_shitai: bool,
    mottomotto_shitai: bool,
    has_yume: bool,
}

impl Lyric {
    fn is_yume(&self) -> bool {
        if !self.aremo_shitai {
            return false;
        }
        if !self.koremo_shitai {
            return false;
        }
        if !self.motto_shitai {
            return false;
        }
        if !self.mottomotto_shitai {
            return false;
        }
        if !self.has_yume {
            return false;
        }
        true
    }
}

struct LyricBuilder {
    aremo_shitai: bool,
    koremo_shitai: bool,
    motto_shitai: bool,
    mottomotto_shitai: bool,
    has_yume: bool,
}

impl LyricBuilder {
    fn new() -> LyricBuilder {
        LyricBuilder {
            aremo_shitai: false,
            koremo_shitai: false,
            motto_shitai: false,
            mottomotto_shitai: false,
            has_yume: false,
        }
    }
    fn aremo_shitai(&mut self, b: bool) -> &mut LyricBuilder {
        self.aremo_shitai = b;
        self
    }
    fn koremo_shitai(&mut self, b: bool) -> &mut LyricBuilder {
        self.koremo_shitai = b;
        self
    }
    fn motto_shitai(&mut self, b: bool) -> &mut LyricBuilder {
        self.motto_shitai = b;
        self
    }

    fn mottomotto_shitai(&mut self, b: bool) -> &mut LyricBuilder {
        self.mottomotto_shitai = b;
        self
    }

    fn has_yume(&mut self, b: bool) -> &mut LyricBuilder {
        self.has_yume = b;
        self
    }
    fn finalize(&self) -> Lyric {
        Lyric {
            aremo_shitai: self.aremo_shitai,
            koremo_shitai: self.koremo_shitai,
            motto_shitai: self.motto_shitai,
            mottomotto_shitai: self.mottomotto_shitai,
            has_yume: self.has_yume,
        }
    }
}

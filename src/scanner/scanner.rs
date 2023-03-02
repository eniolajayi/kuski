struct Scanner {
    source: &String,
    tokens: Vec<Token>,
}

impl Scanner {
    fn new(&mut self, source: &String, tokens: Vec<Token>) -> Scanner {
        Scanner { source, tokens }
    }

    fn set_src(&mut self, source: &String) {
        self.source = *source;
    }

    fn scan_tokens(&mut self)-> Vec<Token>{
        
    }
}

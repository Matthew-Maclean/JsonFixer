use std::io::
{
    Read,
    Result as IOResult,
};

const SLASH: u8 = 0x2F; // '/'
const STAR : u8 = 0x2A; // '*'
const BSLSH: u8 = 0x5C; // '\'
const QUOTE: u8 = 0x22; // '"'
const NEWLN: u8 = 0x0A; // '<newline>'
const COMMA: u8 = 0x2C; // ','
const CLSQR: u8 = 0x5D; // ']'
const CLBRC: u8 = 0x7D; // '}'
const WTSPC: [u8; 4] = [0x20, 0x09, 0x0A, 0x0D]; // JSON whitespace

pub struct JsonFixer<R: Sized>
{
    reader: R,

    state: State,
}

impl<R: Read + Sized> Read for JsonFixer<R>
{
    fn read(&mut self, buf: &mut [u8]) -> IOResult<usize>
    {
        let mut len = 0usize;

        while len < buf.len()
        {
            match self.advance()?
            {
                AdvanceResult::Empty => break,
                AdvanceResult::Nothing => { },
                AdvanceResult::Byte(byte) =>
                {
                    buf[len] = byte;
                    len += 1;
                },
            }
        }

        Ok(len)
    }
}

impl<R: Sized> JsonFixer<R>
{
    pub fn new(reader: R) -> JsonFixer<R>
    {
        JsonFixer
        {
            reader: reader,

            state: State::Main,
        }
    }
}

impl<R: Read + Sized> JsonFixer<R>
{
    fn get_byte(&mut self) -> IOResult<Option<u8>>
    {
        let mut byte = [0u8; 1];

        match self.reader.read(&mut byte)?
        {
            0 => Ok(None),
            _ => Ok(Some(byte[0])),
        }
    }

    fn advance(&mut self) -> IOResult<AdvanceResult>
    {
        match self.state
        {
            State::Main => match self.get_byte()?
            {
                None => return Ok(AdvanceResult::Empty),
                Some(SLASH) => self.state = State::MainSlash,
                Some(QUOTE) => self.state = State::MainQuote,
                Some(COMMA) => self.state = State::Comma,
                Some(byte)  => self.state = State::MainChar(byte),
            },
            State::MainChar(byte) =>
            {
                self.state = State::Main;
                return Ok(AdvanceResult::Byte(byte));
            },
            State::MainSlash => match self.get_byte()?
            {
                None => return Ok(AdvanceResult::Empty),
                Some(SLASH) => self.state = State::LineComment,
                Some(STAR)  => self.state = State::MultiComment,
                Some(QUOTE) => self.state = State::MainQuote,
                Some(byte)  => self.state = State::MainEmitSlash(byte),
            },
            State::MainEmitSlash(byte) =>
            {
                self.state = State::MainSlashEmitChar(byte);
                return Ok(AdvanceResult::Byte(SLASH));
            },
            State::MainSlashEmitChar(byte) =>
            {
                self.state = State::Main;
                return Ok(AdvanceResult::Byte(byte));
            },
            State::MainQuote =>
            {
                self.state = State::Quote;
                return Ok(AdvanceResult::Byte(QUOTE));
            },
            State::LineComment => match self.get_byte()?
            {
                None => return Ok(AdvanceResult::Empty),
                Some(NEWLN) => self.state = State::Main,
                Some(_)     => self.state = State::LineComment,
            },
            State::MultiComment => match self.get_byte()?
            {
                None => return Ok(AdvanceResult::Empty),
                Some(STAR) => self.state = State::MultiCommentStar,
                Some(_)    => self.state = State::MultiComment,
            },
            State::MultiCommentStar => match self.get_byte()?
            {
                None => return Ok(AdvanceResult::Empty),
                Some(SLASH) => self.state = State::Main,
                Some(_)     => self.state = State::MultiComment,
            },
            State::Quote => match self.get_byte()?
            {
                None => return Ok(AdvanceResult::Empty),
                Some(QUOTE) => self.state = State::QuoteQuote,
                Some(BSLSH) => self.state = State::QuoteEscape,
                Some(byte)  => self.state = State::QuoteChar(byte),
            },
            State::QuoteChar(byte) =>
            {
                self.state = State::Quote;
                return Ok(AdvanceResult::Byte(byte));
            },
            State::QuoteQuote =>
            {
                self.state = State::Main;
                return Ok(AdvanceResult::Byte(QUOTE));
            },
            State::QuoteEscape =>
            {
                self.state = State::QuoteEscapeGetChar;
                return Ok(AdvanceResult::Byte(BSLSH));
            },
            State::QuoteEscapeGetChar => match self.get_byte()?
            {
                None => return Ok(AdvanceResult::Empty),
                Some(byte) => self.state = State::QuoteEscapeEmitChar(byte),
            },
            State::QuoteEscapeEmitChar(byte) =>
            {
                self.state = State::Quote;
                return Ok(AdvanceResult::Byte(byte));
            },
            State::Comma => match self.get_byte()?
            {
                None => return Ok(AdvanceResult::Empty),
                Some(CLSQR) => self.state = State::CommaCloseSquare,
                Some(CLBRC) => self.state = State::CommaCloseBrace,
                Some(QUOTE) => self.state = State::CommaQuote,
                Some(SLASH) => self.state = State::CommaSlash,
                Some(byte) if WTSPC.contains(&byte)
                    => self.state = State::CommaChar(byte),
                Some(byte)  => self.state = State::CommaBreak(byte),
            },
            State::CommaChar(byte) =>
            {
                self.state = State::Comma;
                return Ok(AdvanceResult::Byte(byte));
            },
            State::CommaCloseSquare =>
            {
                self.state = State::Main;
                return Ok(AdvanceResult::Byte(CLSQR));
            },
            State::CommaCloseBrace =>
            {
                self.state = State::Main;
                return Ok(AdvanceResult::Byte(CLBRC));
            },
            State::CommaBreak(byte) =>
            {
                self.state = State::CommaBreakEmit(byte);
                return Ok(AdvanceResult::Byte(COMMA));
            },
            State::CommaBreakEmit(byte) =>
            {
                self.state = State::Main;
                return Ok(AdvanceResult::Byte(byte));
            },
            State::CommaQuote =>
            {
                self.state = State::CommaQuoteEmit;
                return Ok(AdvanceResult::Byte(COMMA));
            },
            State::CommaQuoteEmit =>
            {
                self.state = State::Quote;
                return Ok(AdvanceResult::Byte(QUOTE));
            },
            State::CommaSlash => match self.get_byte()?
            {
                None => return Ok(AdvanceResult::Empty),
                Some(SLASH) => self.state = State::CommaLineComment,
                Some(STAR)  => self.state = State::CommaMultiComment,
                Some(byte)  => self.state = State::CommaEmitSlash(byte),
            },
            State::CommaLineComment => match self.get_byte()?
            {
                None => return Ok(AdvanceResult::Empty),
                Some(NEWLN) => self.state = State::Comma,
                Some(_)     => self.state = State::CommaLineComment,
            },
            State::CommaMultiComment => match self.get_byte()?
            {
                None => return Ok(AdvanceResult::Empty),
                Some(STAR) => self.state = State::CommaMultiCommentStar,
                Some(_)    => self.state = State::CommaMultiComment,
            },
            State::CommaMultiCommentStar => match self.get_byte()?
            {
                None => return Ok(AdvanceResult::Empty),
                Some(SLASH) => self.state = State::Comma,
                Some(_)     => self.state = State::CommaMultiComment,
            },
            State::CommaEmitSlash(byte) =>
            {
                self.state = State::CommaSlashEmitChar(byte);
                return Ok(AdvanceResult::Byte(SLASH));
            },
            State::CommaSlashEmitChar(byte) =>
            {
                self.state = State::Comma;
                return Ok(AdvanceResult::Byte(byte));
            },
        }

        Ok(AdvanceResult::Nothing)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum AdvanceResult
{
    Empty,
    Nothing,
    Byte(u8),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum State
{
    Main,
        MainChar(u8),
        //  -> Main
        MainSlash,
        //  MainSlashSlash -> LineComment
        //  MainSlashStar -> MultiComment
            MainEmitSlash(u8),
                MainSlashEmitChar(u8),
                //  -> Main
        MainQuote,
        //  -> Quote
    //  MainComma -> Comma

    LineComment,
    //  LineCommmentNewline -> Main
    //  LineCommentChar -> LineComment
    
    MultiComment,
        MultiCommentStar,
        //  MultiCommentStarChar -> MultiComment
        //  MultiCommentStarSlash -> Main

    Quote,
        QuoteQuote,
        //  -> Main
        QuoteChar(u8),
        //  -> Quote
        QuoteEscape,
            QuoteEscapeGetChar,
                QuoteEscapeEmitChar(u8),
                //  -> Quote
    
    Comma,
        CommaChar(u8),
        //  -> Comma
        CommaCloseSquare,
        //  -> Main
        CommaCloseBrace,
        //  -> Main
        CommaBreak(u8),
            CommaBreakEmit(u8),
        //  -> Main
        CommaQuote,
            CommaQuoteEmit,
                // -> Quote
        CommaSlash,
            CommaLineComment,
            CommaMultiComment,
                CommaMultiCommentStar,
                //  -> CommaMultiCommentStarChar -> CommaMultiComment
                //  -> CommaMultiCommentStarSlash -> Comma
            CommaEmitSlash(u8),
                CommaSlashEmitChar(u8),
                //  -> Comma
}

#[cfg(test)]
mod test;

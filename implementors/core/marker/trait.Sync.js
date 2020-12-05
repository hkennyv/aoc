(function() {var implementors = {};
implementors["aho_corasick"] = [{"text":"impl&lt;S&gt; Sync for AhoCorasick&lt;S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;'a, 'b, S&gt; Sync for FindIter&lt;'a, 'b, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;'a, 'b, S&gt; Sync for FindOverlappingIter&lt;'a, 'b, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;'a, R, S&gt; Sync for StreamFindIter&lt;'a, R, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;R: Sync,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Sync for AhoCorasickBuilder","synthetic":true,"types":[]},{"text":"impl Sync for MatchKind","synthetic":true,"types":[]},{"text":"impl Sync for Error","synthetic":true,"types":[]},{"text":"impl Sync for ErrorKind","synthetic":true,"types":[]},{"text":"impl Sync for MatchKind","synthetic":true,"types":[]},{"text":"impl Sync for Config","synthetic":true,"types":[]},{"text":"impl Sync for Builder","synthetic":true,"types":[]},{"text":"impl Sync for Searcher","synthetic":true,"types":[]},{"text":"impl&lt;'s, 'h&gt; Sync for FindIter&lt;'s, 'h&gt;","synthetic":true,"types":[]},{"text":"impl Sync for Match","synthetic":true,"types":[]}];
implementors["memchr"] = [{"text":"impl&lt;'a&gt; Sync for Memchr&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for Memchr2&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for Memchr3&lt;'a&gt;","synthetic":true,"types":[]}];
implementors["regex"] = [{"text":"impl Sync for RegexBuilder","synthetic":true,"types":[]},{"text":"impl Sync for RegexSetBuilder","synthetic":true,"types":[]},{"text":"impl&lt;'t&gt; Sync for Match&lt;'t&gt;","synthetic":true,"types":[]},{"text":"impl Sync for Regex","synthetic":true,"types":[]},{"text":"impl&lt;'r, 't&gt; !Sync for Matches&lt;'r, 't&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'r, 't&gt; !Sync for CaptureMatches&lt;'r, 't&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'r, 't&gt; !Sync for Split&lt;'r, 't&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'r, 't&gt; !Sync for SplitN&lt;'r, 't&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'r&gt; Sync for CaptureNames&lt;'r&gt;","synthetic":true,"types":[]},{"text":"impl Sync for CaptureLocations","synthetic":true,"types":[]},{"text":"impl&lt;'t&gt; Sync for Captures&lt;'t&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'c, 't&gt; Sync for SubCaptureMatches&lt;'c, 't&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a, R:&nbsp;?Sized&gt; Sync for ReplacerRef&lt;'a, R&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;R: Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;'t&gt; Sync for NoExpand&lt;'t&gt;","synthetic":true,"types":[]},{"text":"impl Sync for RegexSet","synthetic":true,"types":[]},{"text":"impl Sync for SetMatches","synthetic":true,"types":[]},{"text":"impl Sync for SetMatchesIntoIter","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for SetMatchesIter&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl Sync for Error","synthetic":true,"types":[]},{"text":"impl Sync for RegexBuilder","synthetic":true,"types":[]},{"text":"impl Sync for RegexSetBuilder","synthetic":true,"types":[]},{"text":"impl Sync for RegexSet","synthetic":true,"types":[]},{"text":"impl Sync for SetMatches","synthetic":true,"types":[]},{"text":"impl Sync for SetMatchesIntoIter","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for SetMatchesIter&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'t&gt; Sync for Match&lt;'t&gt;","synthetic":true,"types":[]},{"text":"impl Sync for Regex","synthetic":true,"types":[]},{"text":"impl&lt;'r&gt; Sync for CaptureNames&lt;'r&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'r, 't&gt; !Sync for Split&lt;'r, 't&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'r, 't&gt; !Sync for SplitN&lt;'r, 't&gt;","synthetic":true,"types":[]},{"text":"impl Sync for CaptureLocations","synthetic":true,"types":[]},{"text":"impl&lt;'t&gt; Sync for Captures&lt;'t&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'c, 't&gt; Sync for SubCaptureMatches&lt;'c, 't&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'r, 't&gt; !Sync for CaptureMatches&lt;'r, 't&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'r, 't&gt; !Sync for Matches&lt;'r, 't&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a, R:&nbsp;?Sized&gt; Sync for ReplacerRef&lt;'a, R&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;R: Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;'t&gt; Sync for NoExpand&lt;'t&gt;","synthetic":true,"types":[]}];
implementors["regex_syntax"] = [{"text":"impl Sync for ParserBuilder","synthetic":true,"types":[]},{"text":"impl !Sync for Parser","synthetic":true,"types":[]},{"text":"impl Sync for Printer","synthetic":true,"types":[]},{"text":"impl Sync for Error","synthetic":true,"types":[]},{"text":"impl Sync for ErrorKind","synthetic":true,"types":[]},{"text":"impl Sync for Span","synthetic":true,"types":[]},{"text":"impl Sync for Position","synthetic":true,"types":[]},{"text":"impl Sync for WithComments","synthetic":true,"types":[]},{"text":"impl Sync for Comment","synthetic":true,"types":[]},{"text":"impl Sync for Ast","synthetic":true,"types":[]},{"text":"impl Sync for Alternation","synthetic":true,"types":[]},{"text":"impl Sync for Concat","synthetic":true,"types":[]},{"text":"impl Sync for Literal","synthetic":true,"types":[]},{"text":"impl Sync for LiteralKind","synthetic":true,"types":[]},{"text":"impl Sync for SpecialLiteralKind","synthetic":true,"types":[]},{"text":"impl Sync for HexLiteralKind","synthetic":true,"types":[]},{"text":"impl Sync for Class","synthetic":true,"types":[]},{"text":"impl Sync for ClassPerl","synthetic":true,"types":[]},{"text":"impl Sync for ClassPerlKind","synthetic":true,"types":[]},{"text":"impl Sync for ClassAscii","synthetic":true,"types":[]},{"text":"impl Sync for ClassAsciiKind","synthetic":true,"types":[]},{"text":"impl Sync for ClassUnicode","synthetic":true,"types":[]},{"text":"impl Sync for ClassUnicodeKind","synthetic":true,"types":[]},{"text":"impl Sync for ClassUnicodeOpKind","synthetic":true,"types":[]},{"text":"impl Sync for ClassBracketed","synthetic":true,"types":[]},{"text":"impl Sync for ClassSet","synthetic":true,"types":[]},{"text":"impl Sync for ClassSetItem","synthetic":true,"types":[]},{"text":"impl Sync for ClassSetRange","synthetic":true,"types":[]},{"text":"impl Sync for ClassSetUnion","synthetic":true,"types":[]},{"text":"impl Sync for ClassSetBinaryOp","synthetic":true,"types":[]},{"text":"impl Sync for ClassSetBinaryOpKind","synthetic":true,"types":[]},{"text":"impl Sync for Assertion","synthetic":true,"types":[]},{"text":"impl Sync for AssertionKind","synthetic":true,"types":[]},{"text":"impl Sync for Repetition","synthetic":true,"types":[]},{"text":"impl Sync for RepetitionOp","synthetic":true,"types":[]},{"text":"impl Sync for RepetitionKind","synthetic":true,"types":[]},{"text":"impl Sync for RepetitionRange","synthetic":true,"types":[]},{"text":"impl Sync for Group","synthetic":true,"types":[]},{"text":"impl Sync for GroupKind","synthetic":true,"types":[]},{"text":"impl Sync for CaptureName","synthetic":true,"types":[]},{"text":"impl Sync for SetFlags","synthetic":true,"types":[]},{"text":"impl Sync for Flags","synthetic":true,"types":[]},{"text":"impl Sync for FlagsItem","synthetic":true,"types":[]},{"text":"impl Sync for FlagsItemKind","synthetic":true,"types":[]},{"text":"impl Sync for Flag","synthetic":true,"types":[]},{"text":"impl Sync for Error","synthetic":true,"types":[]},{"text":"impl Sync for Literals","synthetic":true,"types":[]},{"text":"impl Sync for Literal","synthetic":true,"types":[]},{"text":"impl Sync for Printer","synthetic":true,"types":[]},{"text":"impl Sync for TranslatorBuilder","synthetic":true,"types":[]},{"text":"impl !Sync for Translator","synthetic":true,"types":[]},{"text":"impl Sync for CaseFoldError","synthetic":true,"types":[]},{"text":"impl Sync for Error","synthetic":true,"types":[]},{"text":"impl Sync for ErrorKind","synthetic":true,"types":[]},{"text":"impl Sync for Hir","synthetic":true,"types":[]},{"text":"impl Sync for HirKind","synthetic":true,"types":[]},{"text":"impl Sync for Literal","synthetic":true,"types":[]},{"text":"impl Sync for Class","synthetic":true,"types":[]},{"text":"impl Sync for ClassUnicode","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for ClassUnicodeIter&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl Sync for ClassUnicodeRange","synthetic":true,"types":[]},{"text":"impl Sync for ClassBytes","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Sync for ClassBytesIter&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl Sync for ClassBytesRange","synthetic":true,"types":[]},{"text":"impl Sync for Anchor","synthetic":true,"types":[]},{"text":"impl Sync for WordBoundary","synthetic":true,"types":[]},{"text":"impl Sync for Group","synthetic":true,"types":[]},{"text":"impl Sync for GroupKind","synthetic":true,"types":[]},{"text":"impl Sync for Repetition","synthetic":true,"types":[]},{"text":"impl Sync for RepetitionKind","synthetic":true,"types":[]},{"text":"impl Sync for RepetitionRange","synthetic":true,"types":[]},{"text":"impl Sync for ParserBuilder","synthetic":true,"types":[]},{"text":"impl !Sync for Parser","synthetic":true,"types":[]},{"text":"impl Sync for UnicodeWordError","synthetic":true,"types":[]},{"text":"impl Sync for Utf8Sequence","synthetic":true,"types":[]},{"text":"impl Sync for Utf8Range","synthetic":true,"types":[]},{"text":"impl Sync for Utf8Sequences","synthetic":true,"types":[]}];
implementors["thread_local"] = [{"text":"impl&lt;'a, T&gt; !Sync for CachedIterMut&lt;'a, T&gt;","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; !Sync for CachedIntoIter&lt;T&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a, T&gt; !Sync for IterMut&lt;'a, T&gt;","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; !Sync for IntoIter&lt;T&gt;","synthetic":true,"types":[]},{"text":"impl&lt;T:&nbsp;Send&gt; Sync for CachedThreadLocal&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Send&gt; Sync for ThreadLocal&lt;T&gt;","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()
searchState.loadedDescShard("http_body_util", 0, "Utilities for <code>http_body::Body</code>.\nA data stream created from a <code>Body</code>.\nAn extension trait for <code>http_body::Body</code> adding various …\nA stream created from a <code>Body</code>.\nA collected body produced by <code>BodyExt::collect</code> which …\nSum type with two cases: <code>Left</code> and <code>Right</code>, used if a body …\nA body that is always empty.\nA body that consists of a single chunk.\nA value of type <code>L</code>\nAn error returned when body length exceeds the configured …\nA length limited body.\nA value of type <code>R</code>\nA body created from a <code>Stream</code>.\nAggregate this buffered into a <code>Buf</code>.\nTurn this body into a boxed trait object.\nTurn this body into a boxed trait object that is !Sync.\nTurn this body into <code>Collected</code> body which will collect all …\nCombinators for the <code>Body</code> trait.\nCreate an empty <code>Full</code>.\nReturns a future that resolves to the next <code>Frame</code>, if any.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nTurn this body into <code>BodyDataStream</code>.\nConvert <code>Either</code> into the inner type, if both <code>Left</code> and <code>Right</code> …\nMaps this body’s error value to a different value.\nMaps this body’s frame to a different kind.\nCreate a new <code>Empty</code>.\nCreate a new <code>Full</code>.\nCreate a new <code>Limited</code>.\nCreate a new <code>StreamBody</code>.\nCreate a new <code>BodyStream</code>.\nCreate a new <code>BodyDataStream</code>\nConvert this body into a <code>Bytes</code>.\nIf there is a trailers frame buffered, returns a reference …\nAdd trailers to the body.\nA boxed <code>Body</code> trait object.\nFuture that resolves into a <code>Collected</code>.\nFuture that resolves to the next frame from a <code>Body</code>.\nBody returned by the <code>map_err</code> combinator.\nBody returned by the <code>map_frame</code> combinator.\nA boxed <code>Body</code> trait object that is !Sync.\nAdds trailers to a body.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nGet a mutable reference to the inner body\nGet a mutable reference to the inner body\nGet a pinned mutable reference to the inner body\nGet a pinned mutable reference to the inner body\nGet a reference to the inner body\nGet a reference to the inner body\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nConsume <code>self</code>, returning the inner body\nConsume <code>self</code>, returning the inner body\nCreate a new <code>BoxBody</code>.\nCreate a new <code>UnsyncBoxBody</code>.")
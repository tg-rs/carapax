(function() {
    var type_impls = Object.fromEntries([["carapax",[]],["nonzero_ext",[]],["serde",[]],["serde_core",[]],["tgbot",[]]]);
    if (window.register_type_impls) {
        window.register_type_impls(type_impls);
    } else {
        window.pending_type_impls = type_impls;
    }
})()
//{"start":55,"fragment_lengths":[14,19,13,18,13]}
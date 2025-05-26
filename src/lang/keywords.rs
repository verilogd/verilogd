pub const SV_TOP_KEYWORDS: &[&str] = &["endmodule", "endpackage", "module", "package"];

pub const SV_MOD_OR_PKG_KEYWORDS: &[&str] = &["localparam"];

pub const SV_MOD_KEYWORDS: &[&str] = &[
	"always",
	"always_comb",
	"always_ff",
	"always_latch",
	"assert",
	"assign",
	"begin",
	"end",
];

pub const SV_PKG_KEYWORDS: &[&str] = &["class", "endclass", "function", "endfunction"];

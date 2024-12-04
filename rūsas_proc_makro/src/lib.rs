use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "Kļ" => "Err",
        "Labi" => "Ok",
        "Virkne" => "String",
        "Bibliotēka" => "HashMap",
        "Noklusējums" => "Default",
        "Kļūda" => "Error",
        "Opcija" => "Option",
        "Dažs" => "Some",
        "Neviens" => "None",
        "Rezultāts" => "Result",
        "Pats" => "Self",
        "drukātlīn" => "println",
        "lauzt" => "break",
        "nesinhr" => "async",
        "gaidīt" => "await",
        "ciklēt" => "loop",
        "pārvietot" => "move",
        "kaste" => "crate",
        "neaizsniedzams_kods" => "unreachable_code",
        "kā" => "as",
        "konst" => "const",
        "īpašība" => "trait",
        "nedrošs" | "nedroši" => "unsafe",
        "iekš" => "in",
        "no" => "from",
        "dinamisks" | "dinamiska" => "dyn",
        "iztīt" => "unwrap",
        "noklusējums" => "default",
        "kā_atsauce" => "as_ref",
        "ii" => "io",
        "ārējs" | "ārēja" => "extern",
        "nepatiess" | "meli" => "false",
        "funkcija" => "fn",
        "vecāks" => "super",
        "ievietot" => "insert",
        "iegūt" => "get",
        "atļaut" => "allow",
        "panikot" | "bļe" => "panic",
        "modulis" => "mod",
        "mainīgs" | "mainīga" | "mainīgam" | "mainīgai" => "mut",
        "jauns" | "jauna" => "new",
        "kur" => "where",
        "katrs" | "katram" | "katra" | "katrai" => "for",
        "iegūt_vai_ievietot_ar" => "get_or_insert_with",
        "galvenais" | "galvenā" => "main",
        "publisks" | "publiska" => "pub",
        "neviens" | "neviena" => None?,
        "atgriezt" => "return",
        "implementācija" => "impl",
        "atsauce" => "ref",
        "atbilst" => "match",
        "ja" => "if",
        "citādāk" => "else",
        "pats" => "self",
        "ļaut" => "let",
        "statisks" | "statiska" => "static",
        "struktūra" => "struct",
        "paredzēt" => "expect",
        "kamēr" => "while",
        "izmantot" => "use",
        "uz" => "into",
        "patiess" | "tiesa" => "true",
        "uzskaitījums" => "enum",
        "Grupa" => "Group",
        "Identifikators" => "Ident",
        "ŽetonuPlūsma" => "TokenStream",
        "ŽetonuKoks" => "TokenTree",
        "uz_virkni" => "to_string",
        "kā_virkne" => "as_str",
        // "" => "span", // TODO
        "Vektors" => "Vec",
        "plūsma" => "stream",
        "uzgrūst" => "push",
        "pagarināt" => "extend",
        "atdalītājs" => "delimiter",
        "Punktuācija" => "Punct",
        "Burtisks" => "Literal",
        "proc_makro" => "proc_macro",
        "nesasniedzams_kods" => "unreachable_code",
        _ => &ident_str,
    };

    let new_ident = Ident::new(new_str, ident.span());
    Some(TokenTree::Ident(new_ident))
}

fn replace_tree(tok: TokenTree, out: &mut Vec<TokenTree>) {
    match tok {
        TokenTree::Group(group) => {
            let mut group_elem = Vec::new();
            replace_stream(group.stream(), &mut group_elem);
            let mut new_stream = TokenStream::new();
            new_stream.extend(group_elem);
            out.push(TokenTree::Group(Group::new(group.delimiter(), new_stream)));
        }
        TokenTree::Ident(ident) => {
            if let Some(ident) = replace_ident(ident) {
                out.push(ident);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            out.push(tok);
        }
    }
}

fn replace_stream(ts: TokenStream, out: &mut Vec<TokenTree>) {
    for tok in ts {
        replace_tree(tok, out)
    }
}

#[proc_macro]
pub fn rūsa(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}

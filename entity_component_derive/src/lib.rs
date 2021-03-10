#![recursion_limit="128"]


extern crate proc_macro;
extern crate syn;
#[macro_use] extern crate quote;


#[proc_macro_derive(ComponentSystem, attributes(component))]
pub fn component_set_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {

    let s = input.to_string();

    let ast = syn::parse_derive_input(&s).unwrap();

    let gen = generate_impl(&ast);

    gen.parse().unwrap()

}


fn generate_impl(ast: &syn::DeriveInput) -> quote::Tokens {

    let ident = &ast.ident;

    let generics = &ast.generics;

    let where_clause = &ast.generics.where_clause;

    let fields_methods_impl = generater_field_methods(&ast.body);

    let res = quote!{
        impl #ident #generics #where_clause {

            #(#fields_methods_impl)*

        }

    };

    // panic!("{:#?}", ast.body);
    //panic!("\n\n\n BUT WHAT IS RES\n\n\n{:#?}\n\n\n", res);
    res
}



fn generater_field_methods(body: &syn::Body) -> Vec<quote::Tokens> {
    let mut remove_statements = "".to_owned();


    let mut funs:Vec<quote::Tokens>  = match body {
        &syn::Body::Enum(_)
            => panic!("ComponentSystem can not be implemented for enums"),
        &syn::Body::Struct(syn::VariantData::Unit)
            =>  panic!("ComponentSystem can not be implemented for Unit structs"),
        &syn::Body::Struct(syn::VariantData::Tuple(_))
            => panic!("ComponentSystem can not be implemented for Tuple structs"),
        &syn::Body::Struct(syn::VariantData::Struct(ref s)) => {
            s.iter()
                .map(|field| {
                    let (tokens, field_name) = generater_field_body(field);
                    match field_name {
                        Some(name) => {
                            remove_statements += &(format!("self.{}.remove(&id); ", name))
                        }


                        None => {}
                    };

                    tokens
                })
                .collect()
        }
    };

    //let remove_statements: Vec<String> = field_names.iter().map(|name| name).collect::<String>().as_slice();

    let remove_body = syn::Ident::new(remove_statements);
    let remove_tokens = quote! {
        #[allow(unsued_varaibles)]
        pub fn remove_entity(&mut self, id: usize) {
            #remove_body
        }
    };

    //    panic!("Field names \n\n\n{:?} \n\n\n", remove_tokens);
    funs.push(remove_tokens);
    funs
}


fn generater_field_body(field: &syn::Field) -> (quote::Tokens, Option<String>) {
    let field_name = match field.ident {
        Some(ref i) => format!("{}",i),
        None => String::from(""),
    };

    let component_attr = match field.attrs.iter().filter(|a| a.value.name() == "component").next() {
        Some(a) => a,
        None => return (quote::Tokens::new(), None)
    };

    let component_value_literal = match component_attr.value {
        syn::MetaItem::NameValue(_, ref literal @ syn::Lit::Str(_,_)) => literal,
        _ => panic!("Field {} component attribute value must be and string", field_name),
    };

    let type_name = match component_value_literal {
        syn::Lit::Str(s,_) => s,
        _ => panic!("String was not a string {:?}", component_value_literal),
    };


    let set_signature = syn::Ident::new(format!("pub fn set_{}(&mut self, entity_id: usize, component: {})", field_name, type_name,));


    let set_body = syn::Ident::new(format!("self.{}.insert(entity_id, component);", field_name));


    let get_signature = syn::Ident::new(format!("pub fn get_{}(& self, entity_id: usize) -> Option<&{}> ", field_name, type_name,));


    let get_body = syn::Ident::new(format!("match &self.{}.get(&entity_id) {} Some(e) => Some(*e), None => None {}", field_name, "{", "}"));


    let tokens =  quote! {

        #set_signature {
            #set_body
        }

        #get_signature {
            #get_body
        }
    };


    (tokens, Some(field_name))

}

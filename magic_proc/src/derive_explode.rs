use parsel::{
    parse2,
    quote::quote,
    syn::{
        punctuated::Punctuated, token::SelfType, GenericParam, Ident, ItemStruct, Lifetime,
        LifetimeDef, Path, PredicateType, Token, TraitBound, TraitBoundModifier, Type, TypeParam,
        TypeParamBound, TypePath, WhereClause, WherePredicate,
    },
    TokenStream,
};
use proc_macro_utils::{
    ext::{FieldsExt, IdentExt, LifetimeExt, PathExt, PunctuatedExt, SelfTypeExt},
    CompileErrorTokenStream, CrateId,
};

pub fn main(input: TokenStream) -> Result<TokenStream, CompileErrorTokenStream> {
    let magic_crate_id = CrateId::new("magic")?;
    let magic_trait_path = magic_crate_id
        .to_path()
        .with_segment(Ident::call_site_span("Magic"));
    let magic_trait_bound = TraitBound {
        paren_token: None,
        modifier: TraitBoundModifier::None,
        lifetimes: None,
        path: magic_trait_path,
    };

    let lifetime = Lifetime::call_site_span("'explode");
    let lifetime_def = LifetimeDef::new(lifetime.clone());

    let magic_type_ident = Ident::call_site_span("ExplosionMagic");
    let magic_type_param_bounds = {
        let mut bounds = Punctuated::new();
        bounds.push(TypeParamBound::Trait(magic_trait_bound));
        bounds
    };
    let magic_type_param = TypeParam {
        attrs: Vec::new(),
        ident: magic_type_ident.clone(),
        colon_token: Some(<Token![:]>::default()),
        bounds: magic_type_param_bounds,
        eq_token: Default::default(),
        default: Default::default(),
    };

    let input: ItemStruct = parse2(input)?;
    let input_vis = input.vis;
    let input_ident = &input.ident;
    let field_idents: Punctuated<&Ident, Token![,]> = input.fields.fully_named_idents()?;

    let exploded_ident = Ident::call_site_span(format!("{}{}", input_ident, "Exploded"));

    let exploded_generics = {
        let mut generics = input.generics.clone();
        generics
            .params
            .insert(0, GenericParam::Lifetime(lifetime_def.clone()));
        generics
            .params
            .push(GenericParam::Type(magic_type_param.clone()));

        let where_clause = generics.where_clause.get_or_insert_with(|| WhereClause {
            where_token: <Token![where]>::default(),
            predicates: Punctuated::new(),
        });
        where_clause
            .predicates
            .push(WherePredicate::Type(PredicateType {
                lifetimes: None,
                bounded_ty: Type::Path(TypePath {
                    qself: None,
                    path: Path::from(SelfType::call_site_span()),
                }),
                colon_token: <Token![:]>::default(),
                bounds: Punctuated::new().with_element(TypeParamBound::Lifetime(lifetime.clone())),
            }));
        for field in input.fields.iter() {
            where_clause
                .predicates
                .push(WherePredicate::Type(PredicateType {
                    lifetimes: None,
                    bounded_ty: field.ty.clone(),
                    colon_token: <Token![:]>::default(),
                    bounds: Punctuated::new()
                        .with_element(TypeParamBound::Lifetime(lifetime.clone())),
                }))
        }

        generics
    };

    let (input_impl_generics, input_ty_generics, input_where_clause) =
        input.generics.split_for_impl();
    let (_exploded_impl_generics, exploded_ty_generics, exploded_where_clause) =
        exploded_generics.split_for_impl();

    let exploded_fields: TokenStream = input
        .fields
        .iter()
        .map(|field| {
            let field_vis = &field.vis;
            let field_ident = &field.ident;
            let field_type = &field.ty;
            quote! {
                #field_vis #field_ident: #magic_crate_id::Magical<#lifetime, #magic_type_ident, #field_type>,
            }
        })
        .collect();

    let explode_field_values: TokenStream = field_idents
        .iter()
        .cloned()
        .map(|ident| {
            quote! {
                // SAFETY: `M::FLAVOR == _`, therefore `Magical<M, _> = _`
                #ident: #magic_crate_id::Magical(unsafe { ::std::mem::transmute_copy(&#ident) }),
            }
        })
        .collect();

    let output = quote! {
        #input_vis struct #exploded_ident #exploded_generics {
            #exploded_fields
        }

        impl #input_impl_generics #magic_crate_id::Explode
        for #input_ident #input_ty_generics #input_where_clause
        {
            type Exploded<#lifetime_def, #magic_type_param> = #exploded_ident #exploded_ty_generics #exploded_where_clause;

            fn explode<#lifetime_def, #magic_type_param>(
                this: <#magic_type_ident as #magic_crate_id::Magic>::Type<#lifetime, Self>
            ) -> <Self as #magic_crate_id::Explode>::Exploded<#lifetime, #magic_type_ident>
            {
                let magical = #magic_crate_id::Magical::<#lifetime, #magic_type_ident, Self>(this);
                match #magic_crate_id::Magical::to_concrete(magical) {
                    #magic_crate_id::ConcreteMagical::Owned(concrete) => {
                        let Self { #field_idents } = concrete;
                        #exploded_ident {
                            #explode_field_values
                        }
                    }
                    #magic_crate_id::ConcreteMagical::Ref(concrete) => {
                        let Self { #field_idents } = concrete;
                        #exploded_ident {
                            #explode_field_values
                        }
                    }
                    #magic_crate_id::ConcreteMagical::Mut(concrete) => {
                        let Self { #field_idents } = concrete;
                        #exploded_ident {
                            #explode_field_values
                        }
                    }
                }
            }
        }
    };

    // std::fs::write("EXPANDED.rs", output.to_string());

    Ok(output)
}

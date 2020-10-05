// This file is part of Substrate.

// Copyright (C) 2020 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::pallet::Def;
use syn::spanned::Spanned;

pub fn expand_type_values(def: &mut Def) -> proc_macro2::TokenStream {
	let mut expand = quote::quote!();
	let frame_support = &def.frame_support;

	for type_value in &def.type_values {
		// Remove item from module content
		let item = &mut def.item.content.as_mut().expect("Checked by def").1[type_value.index];
		let span = item.span();
		*item = syn::Item::Verbatim(Default::default());
		
		let vis = &type_value.vis;
		let ident = &type_value.ident;
		let block = &type_value.block;
		let type_ = &type_value.type_;

		let (
			struct_impl_gen,
			struct_use_gen,
		) = match (type_value.has_trait, type_value.has_instance) {
			(true, true) => (
				quote::quote!(T: Config<I>, I),
				quote::quote!(T, I),
			),
			(true, false) => (quote::quote!(T: Config), quote::quote!(T)),
			(false, false) => (quote::quote!(), quote::quote!()),
			(false, true) => unreachable!("Checked by def"),
		};

		expand.extend(quote::quote_spanned!(span =>
			#vis struct #ident<#struct_use_gen>(core::marker::PhantomData<((), #struct_use_gen)>);
			impl<#struct_impl_gen> #frame_support::traits::Get<#type_> for #ident<#struct_use_gen> {
				fn get() -> #type_ #block
			}
		));
	}
	expand
}
use leptos::prelude::*;

use crate::hooks::books::use_staged_book;

#[component]
pub fn BookInfo() -> impl IntoView {
    if let Some(book) = use_staged_book().get() {
        view! {
            <aside
                id="book-info"
                class="w-80 bg-neutral-950/90 border-l border-neutral-800/50 overflow-auto glass-effect"
            >
                <div class="p-6">
                    <div class="mb-6">
                        <div class="aspect-[2/3] bg-gradient-to-br from-red-900 via-red-800 to-red-950 rounded-lg shadow-2xl mb-4 overflow-hidden book-cover relative">
                            <div class="absolute inset-0 flex items-center justify-center">
                                <div class="text-center p-6">
                                    <h3 class="text-2xl font-bold text-white mb-2">{book.author.clone()}</h3>
                                    <p class="text-xs text-neutral-300 mb-6">{book.title.clone()}</p>
                                </div>
                            </div>
                            <div class="absolute top-0 left-0 right-0 h-1 bg-gradient-to-r from-transparent via-white/30 to-transparent"></div>
                        </div>
                    </div>
                    <div class="space-y-4">
                        <div>
                            <h3 class="text-xl font-bold text-neutral-100 mb-1">"The Briar King"</h3>
                            <p class="text-sm text-neutral-400">"by "{book.author.clone()}</p>
                        </div>
                        <div class="flex flex-wrap gap-2">
                            <span class="px-3 py-1 bg-violet-600/20 text-violet-300 rounded-full text-xs font-medium mono">
                                "Fantasy"
                            </span>
                            <span class="px-3 py-1 bg-fuchsia-600/20 text-fuchsia-300 rounded-full text-xs font-medium mono">
                                "Fiction"
                            </span>
                            <span class="px-3 py-1 bg-purple-600/20 text-purple-300 rounded-full text-xs font-medium mono">
                                "Epic"
                            </span>
                            <span class="px-3 py-1 bg-pink-600/20 text-pink-300 rounded-full text-xs font-medium mono">
                                "Magic"
                            </span>
                            <span class="px-3 py-1 bg-rose-600/20 text-rose-300 rounded-full text-xs font-medium mono">
                                "Medieval"
                            </span>
                            <span class="px-3 py-1 bg-indigo-600/20 text-indigo-300 rounded-full text-xs font-medium mono">
                                "Novels"
                            </span>
                        </div>
                        <div class="pt-4 border-t border-neutral-800/50 space-y-3">
                            <div class="flex justify-between text-sm">
                                <span class="text-neutral-500 mono">"Published:"</span>
                                <span class="text-neutral-300 mono">"25 Feb"</span>
                            </div>
                            <div class="flex justify-between text-sm">
                                <span class="text-neutral-500 mono">"Series:"</span>
                                <span class="text-neutral-300">"The Kingdoms of Thorn and Bone"</span>
                            </div>
                            <div class="flex justify-between text-sm">
                                <span class="text-neutral-500 mono">"Rating:"</span>
                                <div class="flex">
                                    {(0..5)
                                        .map(|i| {
                                            view! {
                                                <span class=if i < 4 {
                                                    "text-yellow-500"
                                                } else {
                                                    "text-neutral-700"
                                                }>"★"</span>
                                            }
                                        })
                                        .collect::<Vec<_>>()}
                                </div>
                            </div>
                            <div class="flex justify-between text-sm">
                                <span class="text-neutral-500 mono">"Format:"</span>
                                <span class="text-neutral-300 mono">"EPUB"</span>
                            </div>
                            <div class="flex justify-between text-sm">
                                <span class="text-neutral-500 mono">"Size:"</span>
                                <span class="text-neutral-300 mono">"2.4 MB"</span>
                            </div>
                        </div>
                        <div class="pt-4 space-y-2">
                            <button class="w-full bg-gradient-to-r from-violet-600 to-purple-600 hover:from-violet-500 hover:to-purple-500 text-white font-medium py-3 rounded-lg transition-all hover-lift shadow-lg shadow-violet-900/50">
                                "Read Now"
                            </button>
                            <button class="w-full bg-neutral-800/50 hover:bg-neutral-800 text-neutral-200 font-medium py-3 rounded-lg transition-all border border-neutral-700/50">
                                "Edit Metadata"
                            </button>
                        </div>
                    </div>
                </div>
            </aside>
        }.into_any();
    }

    let _: () = view! {};
    ().into_any()
}

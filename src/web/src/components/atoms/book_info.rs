use leptos::prelude::*;

use crate::contexts::books::Book;

#[component]
pub fn BookInfo(book: Book) -> impl IntoView {
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
                                <h3 class="text-2xl font-bold text-white mb-2">
                                    {book.title.clone()}
                                </h3>
                                <p class="text-xs text-neutral-300 mb-6">{book.author.clone()}</p>
                            </div>
                        </div>
                        <div class="absolute top-0 left-0 right-0 h-1 bg-gradient-to-r from-transparent via-white/30 to-transparent"></div>
                    </div>
                </div>
                <div class="space-y-4">
                    <div>
                        <h3 class="text-xl font-bold text-neutral-100 mb-1">
                            {book.title.clone()}
                        </h3>
                        <p class="text-sm text-neutral-400">"by "{book.author.clone()}</p>
                    </div>
                    <div class="flex flex-wrap gap-2">
                        <For
                            each=move || book.keywords.clone()
                            key=|kw| kw.clone()
                            children=|kw| {
                                view! {
                                    <span class="px-3 py-1 bg-violet-600/20 text-violet-300 rounded-full text-xs font-medium mono">
                                        {kw}
                                    </span>
                                }
                            }
                        />
                    </div>
                    <div class="pt-4 border-t border-neutral-800/50 space-y-3">
                        <div class="flex justify-between text-sm">
                            <span class="text-neutral-500 mono">"Published:"</span>
                            <span class="text-neutral-300 mono">{book.date}</span>
                        </div>
                        <div class="flex justify-between text-sm">
                            <span class="text-neutral-500 mono">"Format:"</span>
                            <span class="text-neutral-300 mono">{book.format}</span>
                        </div>
                        <div class="flex justify-between text-sm">
                            <span class="text-neutral-500 mono">"Size:"</span>
                            <span class="text-neutral-300 mono">{book.size}</span>
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
    }
}

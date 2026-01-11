use leptos::prelude::*;

use crate::components::atoms::book_info::BookInfo;
use crate::components::atoms::library::Library;
use crate::hooks::app::use_stage;

#[component]
pub fn Manager() -> impl IntoView {
    let (_selected_category, set_selected_category) = signal("All books".to_string());
    let (_selected_book_id, _set_selected_book_id) = signal(Some(82u32));
    let (search_query, set_search_query) = signal(String::new());
    let (sort_by, set_sort_by) = signal("name".to_string());

    let stage = use_stage();
    let categories = vec![
        ("Authors", "85"),
        ("Series", "0"),
        ("Languages", "11"),
        ("Publishers", "44"),
        ("Ratings", "1"),
        ("News", "4"),
        ("Tags", "266"),
    ];

    view! {
        <div class="flex h-screen bg-gradient-to-br from-neutral-950 via-neutral-900 to-neutral-950 text-neutral-100 grid-overlay">
            <aside class="w-64 bg-neutral-950/90 border-r border-neutral-800/50 flex flex-col glass-effect slide-in-left">
                <div class="p-6 border-b border-neutral-800/50">
                    <h1 class="text-3xl font-bold tracking-tight bg-gradient-to-r from-violet-400 via-purple-400 to-fuchsia-400 bg-clip-text text-transparent">
                        "Bookworm"
                    </h1>
                    <p class="text-xs text-neutral-500 mt-1 mono">"v1.0.0.pre • Leo Borai"</p>
                </div>
                <div class="p-4 border-b border-neutral-800/50">
                    <div class="relative">
                        <input
                            type="text"
                            placeholder="Search books..."
                            class="w-full bg-neutral-900/50 border border-neutral-700/50 rounded-lg px-3 py-2 text-sm placeholder-neutral-600 focus:outline-none focus:ring-2 focus:ring-violet-500/50 transition-all mono"
                            prop:value=move || search_query.get()
                            on:input=move |ev| set_search_query.set(event_target_value(&ev))
                        />
                        <svg
                            class="absolute right-3 top-2.5 w-4 h-4 text-neutral-600"
                            fill="none"
                            stroke="currentColor"
                            viewBox="0 0 24 24"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
                            ></path>
                        </svg>
                    </div>
                </div>
                <nav class="flex-1 overflow-y-auto p-4">
                    <div class="space-y-1">
                        <button
                            class="w-full text-left px-3 py-2 rounded-lg transition-all text-sm font-medium bg-violet-600/20 text-violet-300 border-l-2 border-violet-500"
                            on:click=move |_| set_selected_category.set("All books".to_string())
                        >
                            <div class="flex items-center justify-between">
                                <span>"📚 All books"</span>
                                <span class="mono text-xs text-neutral-500">"98"</span>
                            </div>
                        </button>

                        {categories
                            .into_iter()
                            .map(|(name, count)| {
                                let name_clone = name.to_string();
                                view! {
                                    <button
                                        class="w-full text-left px-3 py-2 rounded-lg hover:bg-neutral-800/50 transition-all text-sm text-neutral-400 hover:text-neutral-200 group"
                                        on:click=move |_| {
                                            set_selected_category.set(name_clone.clone())
                                        }
                                    >
                                        <div class="flex items-center justify-between">
                                            <span class="group-hover:translate-x-1 transition-transform">
                                                {format!("📁 {}", name)}
                                            </span>
                                            <span class="mono text-xs text-neutral-600 group-hover:text-neutral-500">
                                                {count}
                                            </span>
                                        </div>
                                    </button>
                                }
                            })
                            .collect::<Vec<_>>()}
                    </div>
                </nav>
                <div class="p-4 border-t border-neutral-800/50">
                    <label class="text-xs text-neutral-500 mb-2 block mono">"Sort by"</label>
                    <select
                        class="w-full bg-neutral-900/50 border border-neutral-700/50 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-violet-500/50 transition-all mono"
                        prop:value=move || sort_by.get()
                        on:change=move |ev| set_sort_by.set(event_target_value(&ev))
                    >
                        <option value="name">"Name"</option>
                        <option value="author">"Author"</option>
                        <option value="date">"Date"</option>
                        <option value="series">"Series"</option>
                    </select>

                    <label class="flex items-center mt-3 text-xs text-neutral-400 cursor-pointer hover:text-neutral-300 transition-colors">
                        <input
                            type="checkbox"
                            class="mr-2 rounded bg-neutral-900 border-neutral-700"
                        />
                        <span class="mono">"Manage user categories"</span>
                    </label>
                </div>
            </aside>
            <main class="flex-1 flex flex-col overflow-hidden">
                <header class="bg-neutral-900/50 border-b border-neutral-800/50 p-4 glass-effect">
                    <div class="flex items-center justify-between">
                        <div class="flex items-center space-x-2">
                            <button class="p-2 hover:bg-neutral-800/50 rounded-lg transition-all group">
                                <svg
                                    class="w-5 h-5 text-neutral-400 group-hover:text-violet-400 transition-colors"
                                    fill="none"
                                    stroke="currentColor"
                                    viewBox="0 0 24 24"
                                >
                                    <path
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        stroke-width="2"
                                        d="M12 6v6m0 0v6m0-6h6m-6 0H6"
                                    ></path>
                                </svg>
                            </button>
                            <button class="p-2 hover:bg-neutral-800/50 rounded-lg transition-all group">
                                <svg
                                    class="w-5 h-5 text-neutral-400 group-hover:text-violet-400 transition-colors"
                                    fill="none"
                                    stroke="currentColor"
                                    viewBox="0 0 24 24"
                                >
                                    <path
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        stroke-width="2"
                                        d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z"
                                    ></path>
                                </svg>
                            </button>
                            <button class="p-2 hover:bg-neutral-800/50 rounded-lg transition-all group">
                                <svg
                                    class="w-5 h-5 text-neutral-400 group-hover:text-red-400 transition-colors"
                                    fill="none"
                                    stroke="currentColor"
                                    viewBox="0 0 24 24"
                                >
                                    <path
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        stroke-width="2"
                                        d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"
                                    ></path>
                                </svg>
                            </button>
                            <div class="w-px h-6 bg-neutral-800"></div>
                            <button class="p-2 hover:bg-neutral-800/50 rounded-lg transition-all group">
                                <span class="text-sm text-neutral-400 group-hover:text-violet-400 transition-colors mono">
                                    "💾"
                                </span>
                            </button>
                            <button class="p-2 hover:bg-neutral-800/50 rounded-lg transition-all group">
                                <span class="text-sm text-neutral-400 group-hover:text-violet-400 transition-colors mono">
                                    "📖"
                                </span>
                            </button>
                            <button class="p-2 hover:bg-neutral-800/50 rounded-lg transition-all group">
                                <span class="text-sm text-neutral-400 group-hover:text-violet-400 transition-colors mono">
                                    "🔄"
                                </span>
                            </button>
                        </div>
                        <div class="flex items-center space-x-2">
                            <div class="status-badge px-3 py-1 bg-emerald-500/20 text-emerald-400 rounded-full text-xs font-medium mono">
                                "Jobs: 0"
                            </div>
                            <button class="p-2 hover:bg-neutral-800/50 rounded-lg transition-all group">
                                <svg
                                    class="w-5 h-5 text-neutral-400 group-hover:text-violet-400 transition-colors"
                                    fill="none"
                                    stroke="currentColor"
                                    viewBox="0 0 24 24"
                                >
                                    <path
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        stroke-width="2"
                                        d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"
                                    ></path>
                                    <path
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        stroke-width="2"
                                        d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
                                    ></path>
                                </svg>
                            </button>
                        </div>
                    </div>
                </header>
                <div class="flex flex-1 overflow-hidden">
                    <Library />
                    <Show when=move || stage.get().is_some()>
                        <BookInfo />
                    </Show>
                </div>
            </main>
        </div>
    }
}

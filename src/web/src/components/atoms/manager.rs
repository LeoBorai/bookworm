use leptos::prelude::*;

use crate::components::atoms::book_info::BookInfo;
use crate::components::atoms::file_upload_button::FileUploadButton;
use crate::components::atoms::library::Library;
use crate::hooks::app::use_stage;
use crate::hooks::books::use_staged_book;

#[component]
pub fn Manager() -> impl IntoView {
    let (_selected_category, _set_selected_category) = signal("All books".to_string());
    let (_selected_book_id, _set_selected_book_id) = signal(Some(82u32));
    let (_search_query, _set_search_query) = signal(String::new());
    let (_sort_by, _set_sort_by) = signal("name".to_string());

    let _stage = use_stage();
    let _categories = [
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
            <main class="flex-1 flex flex-col overflow-hidden">
                <header class="bg-neutral-900/50 border-b border-neutral-800/50 p-4 glass-effect">
                    <div class="flex items-center justify-between">
                        <div>
                            <h1 class="text-2xl font-bold tracking-tight bg-gradient-to-r from-violet-400 via-purple-400 to-fuchsia-400 bg-clip-text text-transparent">
                                "Bookworm"
                            </h1>
                            <p class="text-xs space-x-0.5 text-neutral-500 mt-1 mono">
                                <code>"v1.0.0.pre"</code>
                                <i>"•"</i>
                                <span>"Leo Borai"</span>
                            </p>
                        </div>
                        <div class="flex items-center space-x-2">
                            <button id="upload-button">
                                <FileUploadButton class="p-2 hover:bg-neutral-800/50 rounded-lg transition-all group">
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
                                </FileUploadButton>
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
                    <Show when=move || {
                        use_staged_book().get().is_some()
                    }>
                        {
                            let book = use_staged_book().get().expect("This should not occur");

                            view! { <BookInfo book=book /> }
                        }
                    </Show>
                </div>
            </main>
        </div>
    }
}

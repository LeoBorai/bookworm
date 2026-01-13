use leptos::prelude::*;

use crate::hooks::app::use_set_stage;
use crate::hooks::books::use_library;

#[component]
pub fn Library() -> impl IntoView {
    let library = use_library();

    view! {
        <div class="flex-1 overflow-auto fade-in">
            <div class="p-4">
                <div class="bg-neutral-900/30 rounded-lg overflow-hidden border border-neutral-800/50">
                    <table class="w-full">
                        <thead class="bg-neutral-900/70 sticky top-0">
                            <tr class="border-b border-neutral-800/50">
                                <th class="text-left px-4 py-3 text-xs font-semibold text-neutral-400 uppercase tracking-wider mono">
                                    "#"
                                </th>
                                <th class="text-left px-4 py-3 text-xs font-semibold text-neutral-400 uppercase tracking-wider mono">
                                    "Title"
                                </th>
                                <th class="text-left px-4 py-3 text-xs font-semibold text-neutral-400 uppercase tracking-wider mono">
                                    "Author(s)"
                                </th>
                                <th class="text-left px-4 py-3 text-xs font-semibold text-neutral-400 uppercase tracking-wider mono">
                                    "Date"
                                </th>
                            </tr>
                        </thead>
                        <tbody>
                            <For
                                each=move || library.get()
                                key=|book| book.id
                                children=move |book| {
                                    let handle_click = {
                                        let set_stage = use_set_stage();
                                        let book_id = book.id;
                                        move |_| {
                                            set_stage(book_id);
                                        }
                                    };
                                    view! {
                                        <tr on:click=handle_click>
                                            <td class="px-4 py-3 text-sm text-neutral-500 mono">
                                                {book.id}
                                            </td>
                                            <td class="px-4 py-3 text-sm text-neutral-200 font-medium">
                                                {book.title}
                                            </td>
                                            <td class="px-4 py-3 text-sm text-neutral-400">
                                                {book.author}
                                            </td>
                                            <td class="px-4 py-3 text-sm text-neutral-500 mono">
                                                {book.date}
                                            </td>
                                        </tr>
                                    }
                                }
                            />
                        </tbody>
                    </table>
                </div>
            </div>
        </div>
    }
}

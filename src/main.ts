// Scaffold only. No commands are wired up yet.
//
// Once you add Tauri commands in src-tauri/src/lib.rs (M2: save_question,
// list_questions, delete_question — M4: draft_answer), call them from here
// with invoke(), e.g.:
//
//   import { invoke } from "@tauri-apps/api/core";
//   const saved = await invoke("save_question", { asker, context, question, tags });
//
// See https://tauri.app/develop/calling-rust/ for the full pattern.

import { invoke } from "@tauri-apps/api/core";

interface Draft {
  draft: string;
  verify: string[];
}

interface Question {
  id: string;
  asker: string;
  question: string;
  context: string;
  tags: string[];
  draft: Draft | null;
}
window.addEventListener("DOMContentLoaded", () => {
  const form = document.querySelector("form")!;
  const tagInput = document.querySelector<HTMLInputElement>("#tag-input")!;
  const tbody = document.querySelector<HTMLTableSectionElement>("#questions-tbody")!;
  const noQuestionsMsg = document.querySelector<HTMLHeadingElement>("#no-questions")!;

  const showNoQuestionsMessage = (message: string) => {
    noQuestionsMsg.textContent = message;
    noQuestionsMsg.style.display = "block";
  };

  const hideNoQuestionsMessage = () => {
    noQuestionsMsg.textContent = "No Questions found";
    noQuestionsMsg.style.display = "none";
  };

  async function loadQuestions() {
    const questions = await invoke<Question[]>("list_questions");

    tbody.innerHTML = "";
    if (questions.length === 0){
      showNoQuestionsMessage("No Questions found");
      return;
    }
    hideNoQuestionsMessage();

    questions.forEach((q) => {
      const row = document.createElement("tr");
      row.innerHTML = `
        <td>${q.question}</td>
        <td>${q.asker}</td>
        <td>${q.context}</td>
        <td>${q.tags.join(", ")}</td>
        <td>${q.draft ? q.draft.draft : "No draft yet"}</td>
        <td>
          <button class="delete-btn" data-id="${q.id}">Delete</button>
          <button class="draft-btn" data-id="${q.id}">Draft</button>
        </td>
      
        `;
        tbody.appendChild(row);
    });
  }
  
  
  form.addEventListener("submit", async (e) => {
    e.preventDefault();

    const formData = new FormData(form);
    const asker = formData.get("asker") as string;
    const question = formData.get("question") as string;
    const context = formData.get("context") as string;

    const tags = tagInput.value
      .split(",")
      .map(t => t.trim())
      .filter(Boolean);

    try {
      const saved = await invoke<string>("save_question", { asker, context, question, tags });
      console.log(saved);
      form.reset();
      await loadQuestions();
    } catch (err) {
      console.error("Failed to save question:", err);
    }
  });
  tbody.addEventListener("click", async (e) => {
      const target = e.target as HTMLElement;
      if (target.classList.contains("delete-btn")) {
        const id = target.dataset.id;
        try {
          await invoke("delete_question", { id })
          await loadQuestions();
        } catch (err) {
          console.error("Failed to delete question:", err);
        }
      }
      if (target.classList.contains("draft-btn")) {
        const id = target.dataset.id;
        try {
          await invoke("draft_answer", { id })
          await loadQuestions();
        } catch (err) {
          const message = err instanceof Error ? err.message : String(err);
          console.error("Failed to draft question:", err);
          showNoQuestionsMessage(message);
        }
      }
  });
   loadQuestions();
});



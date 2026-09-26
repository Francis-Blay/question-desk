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

const getDraftPreview = (draft: string) => {
  if (!draft) return "No draft yet";

  const cleaned = draft.replace(/\s+/g, " ").trim();
  const sentences = cleaned.match(/[^.!?]+[.!?]+(?:\s+|$)/g) ?? [cleaned];
  const previewSentences = sentences.slice(0, 2).join(" ").trim();

  if (previewSentences.length >= cleaned.length) {
    return cleaned;
  }

  return `${previewSentences}...`;
};

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
      const draftText = q.draft ? q.draft.draft : "";
      const row = document.createElement("tr");
      row.innerHTML = `
        <td>${q.question}</td>
        <td>${q.asker}</td>
        <td>${q.context}</td>
        <td>${q.tags.join(", ")}</td>
        <td class="draft-cell">
          ${draftText ? `
            <span class="draft-preview" style="display: block;">${getDraftPreview(draftText)}</span>
            <span class="draft-full" style="display: none;">${draftText}</span>
          ` : "No draft yet"}
        </td>
        <td>
          <button class="delete-btn" data-id="${q.id}">Delete</button>
          <button class="view-draft-btn" data-id="${q.id}" data-expanded="false" ${draftText ? "" : "disabled"}>View draft</button>
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

      const viewDraftButton = target.closest(".view-draft-btn") as HTMLButtonElement | null;
      if (viewDraftButton) {
        const row = viewDraftButton.closest("tr");
        const draftCell = row?.querySelector(".draft-cell");
        const preview = draftCell?.querySelector(".draft-preview") as HTMLElement | null;
        const fullDraft = draftCell?.querySelector(".draft-full") as HTMLElement | null;

        if (!preview || !fullDraft) {
          return;
        }

        const isExpanded = viewDraftButton.dataset.expanded === "true";
        preview.style.display = isExpanded ? "block" : "none";
        fullDraft.style.display = isExpanded ? "none" : "block";
        viewDraftButton.dataset.expanded = String(!isExpanded);
        viewDraftButton.textContent = isExpanded ? "View draft" : "Hide draft";
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



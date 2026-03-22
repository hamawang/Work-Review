import { writable } from 'svelte/store';

export const BASIC_ASSISTANT_MODEL_ID = '__basic__';

const STORAGE_KEY = 'work-review-assistant-state';
const DEFAULT_STATE = {
  messages: [],
};

function loadState() {
  if (typeof window === 'undefined') {
    return DEFAULT_STATE;
  }

  try {
    const raw = window.localStorage.getItem(STORAGE_KEY);
    if (!raw) {
      return DEFAULT_STATE;
    }

    const parsed = JSON.parse(raw);
    return {
      messages: Array.isArray(parsed?.messages) ? parsed.messages : [],
    };
  } catch (error) {
    console.warn('加载助手会话缓存失败:', error);
    return DEFAULT_STATE;
  }
}

function persistState(state) {
  if (typeof window === 'undefined') {
    return;
  }

  try {
    window.localStorage.setItem(STORAGE_KEY, JSON.stringify(state));
  } catch (error) {
    console.warn('保存助手会话缓存失败:', error);
  }
}

function createAssistantStore() {
  const { subscribe, set, update } = writable(loadState());

  subscribe((state) => {
    persistState(state);
  });

  return {
    subscribe,
    appendMessage: (message) =>
      update((state) => ({
        ...state,
        messages: [...state.messages, message].slice(-40),
      })),
    clearMessages: () =>
      update((state) => ({
        ...state,
        messages: [],
      })),
    setMessages: (messages) =>
      update((state) => ({
        ...state,
        messages: Array.isArray(messages) ? messages.slice(-40) : [],
      })),
    reset: () => set(DEFAULT_STATE),
  };
}

export const assistantStore = createAssistantStore();

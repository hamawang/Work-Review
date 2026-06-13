import { writable } from 'svelte/store';

export const BASIC_ASSISTANT_MODEL_ID = '__basic__';

const STORAGE_KEY = 'work-review-assistant-state';
const DEFAULT_STATE = {
  messages: [],
  selectedModelId: BASIC_ASSISTANT_MODEL_ID,
  sending: false,
};

function genId() {
  if (typeof crypto !== 'undefined' && crypto.randomUUID) return crypto.randomUUID();
  return `m-${Date.now()}-${Math.random().toString(36).slice(2)}`;
}

function normalizeMessage(message) {
  return {
    ...message,
    id: message?.id || genId(),
    cards: Array.isArray(message?.cards) ? message.cards : [],
    references: Array.isArray(message?.references) ? message.references : [],
    toolLabels: Array.isArray(message?.toolLabels) ? message.toolLabels : [],
    steps: Array.isArray(message?.steps) ? message.steps : [],
    streaming: Boolean(message?.streaming),
  };
}

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
      messages: Array.isArray(parsed?.messages)
        ? parsed.messages.map((message) => normalizeMessage(message))
        : [],
      selectedModelId:
        typeof parsed?.selectedModelId === 'string' && parsed.selectedModelId.trim()
          ? parsed.selectedModelId
          : BASIC_ASSISTANT_MODEL_ID,
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

  let _persistTimer = null;
  subscribe((state) => {
    if (_persistTimer) clearTimeout(_persistTimer);
    _persistTimer = setTimeout(() => {
      _persistTimer = null;
      persistState(state);
    }, 500);
  });

  return {
    subscribe,
    appendMessage: (message) =>
      update((state) => ({
        ...state,
        messages: [...state.messages, normalizeMessage(message)].slice(-40),
      })),
    clearMessages: () =>
      update((state) => ({
        ...state,
        messages: [],
      })),
    setSelectedModelId: (selectedModelId) =>
      update((state) => ({
        ...state,
        selectedModelId:
          typeof selectedModelId === 'string' && selectedModelId.trim()
            ? selectedModelId
            : BASIC_ASSISTANT_MODEL_ID,
      })),
    setMessages: (messages) =>
      update((state) => ({
        ...state,
        messages: Array.isArray(messages)
          ? messages.slice(-40).map((message) => normalizeMessage(message))
          : [],
      })),
    setSending: (sending) =>
      update((state) => ({ ...state, sending })),
    // 增量更新当前 streaming 的 assistant message（流式事件驱动）。
    updateLastStreaming: (updater) =>
      update((state) => {
        const idx = state.messages.findIndex((m) => m.streaming);
        if (idx === -1) return state;
        const newMessages = state.messages.slice();
        newMessages[idx] = normalizeMessage(updater({ ...newMessages[idx] }));
        return { ...state, messages: newMessages };
      }),
    reset: () => set(DEFAULT_STATE),
  };
}

export const assistantStore = createAssistantStore();

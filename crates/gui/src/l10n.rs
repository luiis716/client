//! Portuguese UI copy for this fork — the one place upstream pulls should not
//! overwrite.
//!
//! ## Why this file exists
//!
//! Upstream ships English literals next to the widgets. Editing those strings
//! in place works until the next `git pull`, which reintroduces English on
//! every touched line and fights our translations in the same hunk as new
//! features.
//!
//! Keep **all** Portuguese the product shows in this module. Call sites use
//! [`tr`] (English key → Portuguese) or the named helpers below. After a pull:
//!
//! 1. `git stash -u && git pull --ff-only origin main && git stash pop`
//! 2. Resolve conflicts preferring *upstream behaviour* + *our* `tr(...)` /
//!    destination helpers — never drop Canais/Grupos or the avatar newsletter
//!    path.
//! 3. Grep for fresh English UI (`tooltip("`, `EmptyState::new("`, `.label("`)
//!    and add only the **new** keys here. Existing entries stay put.
//!
//! Features that are ours (rail destinations, newsletter avatars) live in
//! dedicated code (`status::Destination`, `chat_row::ChatKind`,
//! `session::whatsapp::avatar`) so pulls add behaviour beside them instead of
//! rewriting them.

/// Translate an upstream English UI string.
///
/// Unknown keys fall back to the English literal so a new upstream phrase
/// still compiles and draws until we map it. Keys are `'static` so tooltips
/// and button tips keep their existing signatures.
pub fn tr(english: &'static str) -> &'static str {
    match english {
        // Destinations / filters
        "Chats" => "Conversas",
        "Groups" => "Grupos",
        "Channels" => "Canais",
        "Status" => "Status",
        "All" => "Todas",
        "Unread" => "Não lidas",
        "Archived" => "Arquivadas",

        // Common chrome
        "Settings" => "Configurações",
        "Back to chats" => "Voltar às conversas",
        "Close settings" => "Fechar configurações",
        "Voice call" => "Chamada de voz",
        "Video call" => "Chamada de vídeo",
        "Call back" => "Retornar",
        "React" => "Reagir",
        "Reply" => "Responder",
        "Cancel reply" => "Cancelar resposta",
        "Play" => "Reproduzir",
        "Pause" => "Pausar",
        "Playback speed" => "Velocidade de reprodução",
        "Re-read the file from disk" => "Ler o arquivo do disco de novo",

        // Empty states
        "Nothing unread" => "Nada não lido",
        "No archived chats" => "Nada arquivado",
        "No chats" | "No chats yet" => "Ainda sem conversas",
        "Nothing here under the current filter." => "Nada aqui com o filtro atual.",
        "Show all chats" => "Mostrar todas",
        "No results" => "Nenhum resultado",
        "Clear search" => "Limpar busca",

        // Core fallbacks (also used from oxidezap-core via mirrored copy)
        "Group name unavailable" => "Grupo sem nome",
        "Broadcast list" => "Lista de transmissão",
        "Channel" => "Canal",
        "Unknown contact" => "Contato desconhecido",
        "Unknown chat" => "Conversa desconhecida",

        other => other,
    }
}

/// Rail destination labels — fork-owned vocabulary.
pub fn destination_label(id: &'static str) -> &'static str {
    match id {
        "chats" => tr("Chats"),
        "groups" => tr("Groups"),
        "channels" => tr("Channels"),
        "status" => tr("Status"),
        other => other,
    }
}

/// Chat-list filter chip labels.
pub fn filter_label(id: &'static str) -> &'static str {
    match id {
        "all" => tr("All"),
        "unread" => tr("Unread"),
        "archived" => tr("Archived"),
        other => other,
    }
}

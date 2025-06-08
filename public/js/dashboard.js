
document.addEventListener('DOMContentLoaded', () => {
    const overlay = document.getElementById('modal-overlay');
const content = document.getElementById('modal-content');
function openModal(html) {
    content.innerHTML = html;
        overlay.style.display = 'flex';
    }

function closeModal() {
    overlay.style.display = 'none';
    window.location.reload();
}

overlay.addEventListener('click', e => {
    if (e.target === overlay || e.target === closeBtn) {
        closeModal();
    }
});
    document.querySelectorAll('.refuse-form').forEach(form => {
      form.addEventListener('submit', e => {
        e.preventDefault();
  
        const actionUrl = form.getAttribute('action');
  
        // HTML à injecter dans la modale
        const modalHtml = `
          <h2>Raison du refus</h2>
          <textarea id="refuse-reason" placeholder="Expliquez la raison..." style="width: 100%; height: 100px;"></textarea>
          <div style="margin-top: 10px; text-align: right;">
            <button id="submit-refusal" data-action="${actionUrl}" style="margin-right: 5px;">Envoyer</button>
            <button id="cancel-refusal">Annuler</button>
          </div>
        `;
  
        openModal(modalHtml); // ✅ on utilise ta fonction ici
  
        // Attacher les événements une fois la modale affichée
        setTimeout(() => {
          const submitBtn = document.getElementById('submit-refusal');
          const cancelBtn = document.getElementById('cancel-refusal');
          const reasonInput = document.getElementById('refuse-reason');
  
          submitBtn.addEventListener('click', async () => {
            const reason = reasonInput.value.trim();
            if (!reason) {
              alert("Veuillez entrer une raison.");
              return;
            }
  
            try {
              const response = await fetch(submitBtn.dataset.action, {
                method: 'POST',
                headers: {
                  'Content-Type': 'application/json',
                },
                body: JSON.stringify({ reason })
              });
  
              if (response.ok) {
                alert("Refus envoyé !");
                closeModal(); // ✅ utilise ta fonction
              } else {
                alert("Erreur serveur.");
              }
            } catch (err) {
              console.error(err);
              alert("Erreur réseau.");
            }
          });
  
          cancelBtn.addEventListener('click', closeModal);
        }, 0); // Attendre que le DOM de la modale soit injecté
      });
    });
  });
  
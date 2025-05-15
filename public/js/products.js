document.addEventListener('DOMContentLoaded', () => {

    /* ---------- Modal helpers ---------- */
    const overlay = document.getElementById('modal-overlay');
    const box = document.getElementById('modal-box');
    const content = document.getElementById('modal-content');
    const closeBtn = document.getElementById('modal-close');

    function openModal(html) {
        content.innerHTML = html;
        overlay.style.display = 'flex';
    }
    function closeModal() { overlay.style.display = 'none'; }

    overlay.addEventListener('click', e => {
        if (e.target === overlay || e.target === closeBtn) closeModal();
    });

    /* ---------- Button dispatcher ---------- */
    document.body.addEventListener('click', e => {
        const btn = e.target.closest('[data-action]');
        if (!btn) return;

        const action = btn.dataset.action;
        const id = btn.dataset.id;
        const name = btn.dataset.name;

        if (action === 'delete') {
            openModal(`
        <h2>Delete “${name || id}”?</h2>
        <p>This action cannot be undone.</p>
        <button id="confirm-del">Yes, delete</button>
        <button onclick="(${closeModal})()">Cancel</button>
      `);
            document.getElementById('confirm-del')
                .addEventListener('click', async () => {
                    await fetch('/products/' + id, { method: 'DELETE' });
                    btn.closest('.product-card').remove();
                    closeModal();
                });

        } else if (action === 'edit') {
            openModal('<p>Loading…</p>');
            fetch('/products/' + id)          // expect HTML fragment
                .then(r => r.text())
                .then(html => openModal(html))
                .catch(() => openModal('<p>Error loading form.</p>'));

        } else if (action === 'shop') {
            openModal(`
        <h2>Buy “${name}”</h2>
        <label>Quantity:
          <input type="number" id="qty" value="1" min="1">
        </label>
        <button id="add-cart">Add to cart</button>
      `);
            document.getElementById('add-cart')
                .addEventListener('click', async () => {
                    const qty = +document.getElementById('qty').value || 1;
                    await fetch('/cart', {
                        method: 'POST',
                        headers: { 'Content-Type': 'application/json' },
                        body: JSON.stringify({ id, qty })
                    });
                    closeModal();
                    alert(`${qty} × ${name} added to cart!`);
                });
        }
    });
});


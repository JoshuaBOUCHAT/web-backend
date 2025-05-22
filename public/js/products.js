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

    function closeModal() {
        overlay.style.display = 'none';
        window.location.reload();
    }

    overlay.addEventListener('click', e => {
        if (e.target === overlay || e.target === closeBtn) {
            closeModal();
        }
    });

    /* ---------- Main dispatcher ---------- */
    document.body.addEventListener('click', e => {
        const btn = e.target.closest('[data-action]');
        if (!btn) return;

        const action = btn.dataset.action;
        const id = btn.dataset.id;
        const name = btn.dataset.name;

        if (action === 'delete') {
            handleDelete(id, name);
        } else if (action === 'edit') {
            handleEdit(id);
        } else if (action === 'shop') {
            handleShop(id, name);
        } else if (action === 'add') {
            handleAdd();
        }
    });

    /* ---------- Handlers ---------- */

    function handleDelete(id, name) {
        openModal(`
            <h2>Delete “${name || id}”?</h2>
            <p>This action cannot be undone.</p>
            <button id="confirm-del">Yes, delete</button>
            <button onclick="(${closeModal})()">Cancel</button>
        `);
        document.getElementById('confirm-del')
            .addEventListener('click', async () => {
                try {
                    const res = await fetch('/products/' + id, { method: 'DELETE' });
                    const html = await res.text();
                    openModal(html);
                    // btn.closest('.product-card')?.remove(); // Optional: remove from UI
                    // closeModal();
                } catch {
                    openModal('<p>Error deleting product.</p>');
                }
            });
    }

    function handleEdit(id) {
        openModal('<p>Loading…</p>');
        fetch('/products/' + id)
            .then(r => r.text())
            .then(html => openModal(html))
            .catch(() => openModal('<p>Error loading form.</p>'));
    }

    function handleShop(id, name) {
        openModal(`
            <h2>Buy “${name}”</h2>
            <label>Quantity:
              <input type="number" id="qty" value="1" min="1">
            </label>
            <button id="add-cart">Add to cart</button>
        `);
        console.log(id + " " + name);
        document.getElementById('add-cart')
            .addEventListener('click', async () => {
                const qty = document.getElementById('qty').value;

                const response = await fetch('/order/' + id + '/' + qty, {
                    method: 'PUT',
                    //headers: { 'Content-Type': 'te' },
                });
                console.log("pass here");
                alert(response.text());
                closeModal();
            });
    }
    function handleAdd() {
        openModal(`
        <h2>Add New Product</h2>
        <form id="add-form" enctype="multipart/form-data" method="post" action="/product">
            <label>
                Name:
                <input name="name" required>
            </label><br>

            <label>
                Description:
                <textarea name="description" required></textarea>
            </label><br>

            <label>
                Price (€):
                <input name="price" type="number" step="0.01" required>
            </label><br>

            <label>
                Upload image:
                <input name="image_file" type="file" accept="image/*" id="image-upload" required>
            </label><br>

            <button type="submit">Add Product</button>
        </form>
    `);
    }
    document.body.addEventListener('submit', async (e) => {
        if (e.target.matches('#edit-form')) {
            e.preventDefault();

            const form = e.target;
            const id = form.dataset.id;
            const data = new FormData(form);

            try {
                const r = await fetch(`/products/${id}`, { method: 'PATCH', body: data });
                if (!r.ok) throw new Error(await r.text());

                alert('Produit mis à jour');
                closeModal();
            } catch (err) {
                alert('Erreur : ' + err.message);
            }
        } else if (e.target.matches('#add-form')) {
            e.preventDefault();

            const form = e.target;
            const data = new FormData(form);

            try {
                const r = await fetch('/product', { method: 'POST', body: data });
                if (!r.ok) throw new Error(await r.text());

                alert('Produit ajouté avec succès');
                closeModal();
                // Optionally reload or append the new product to the UI here
            } catch (err) {
                alert('Erreur : ' + err.message);
            }
        }
    });
    document.addEventListener('click', async (e) => {
        const btn = e.target.closest('.visibility-toggle-button');
        if (!btn) return;

        const card = btn.closest('.product-card');
        const id = card.dataset.id;
        const makeVis = btn.dataset.action === 'set-visible';
        const newValue = makeVis ? 1 : 0;          // 1 = visible, 0 = invisible

        try {
            const resp = await fetch(`/product/${id}/visibility`, {
                method: 'PUT',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ visible: newValue })
            });
            if (!resp.ok) throw new Error(await resp.text());

            // Basculer l’affichage localement :
            card.querySelector('.btn-eye-visible').classList.toggle('is-hidden', newValue === 0);
            card.querySelector('.btn-eye-invisible').classList.toggle('is-hidden', newValue === 1);

        } catch (err) {
            alert('Erreur : ' + err.message);
        }
    });


});

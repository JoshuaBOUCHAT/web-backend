document.addEventListener('DOMContentLoaded', () => {
    // Initialiser les cases à cocher des super catégories
    const superCategoryCheckboxes = document.querySelectorAll('.filter-group > label > input[type="checkbox"][data-id]');
    superCategoryCheckboxes.forEach(checkbox => {
        checkbox.checked = true;
        checkbox.addEventListener('change', handleCategoryChange);
    });

    // Initialiser les cases à cocher des sous-catégories
    const subCategoryCheckboxes = document.querySelectorAll('.filter-group__sub input[type="checkbox"][data-id]');
    subCategoryCheckboxes.forEach(checkbox => {
        checkbox.checked = true;
        checkbox.addEventListener('change', handleCategoryChange);
    });

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
        } else if (action === 'openCategoryModal') {
            handleCreateCategory();
        } else if (action === 'edit-category') {
            handleCategoryEdit();
        } else if (action === 'delete-category') {
            e.preventDefault();
            handleDeleteCategory(id);
        }
    });




    function handleCategoryEdit() {
        fetch('/category/select', { method: 'GET', })
            .then(r => r.text())
            .then(html => {
                openModal(html);


            }).catch(() => openModal('<p>Error loading category edit form.</p>'));
    }


    function handleDeleteCategory(id) {
        if (confirm('Voulez-vous vraiment supprimer cette catégorie ?')) {
            fetch('/category/' + id, { method: 'DELETE' })
                .then(r => {
                    if (!r.ok) throw new Error("Erreur lors de la suppression");
                    return r.text();
                })
                .then(html => {
                    alert(html);
                    closeModal();
                })
                .catch(e => {
                    console.error(e);
                    alert('Erreur : ' + e.message);
                });
        }
    }


    /* ---------- Handlers ---------- */
    function handleCategoryChange() {
        const uncheckedCategories = Array.from(subCategoryCheckboxes)
            .filter(checkbox => !checkbox.checked)
            .map(checkbox => checkbox.dataset.id);

        // Sélectionner tous les produits
        handleCategoryChangeWithId(uncheckedCategories);


    }
    function handleCategoryChangeWithId(uncheckedCategories) {
        const products = document.querySelectorAll('.product-card');

        products.forEach(product => {

            if (product.classList.contains('product-card--add')) {
                return;
            }
            // Récupérer les catégories du produit
            const productCategories = product.dataset.categories?.split(',') || [];

            // Vérifier si le produit doit être visible
            const isVisible = !uncheckedCategories.some(categoryId =>
                productCategories.includes(categoryId)
            );

            product.style.display = isVisible ? '' : 'none';
        });
    }



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

    function handleCreateCategory() {
        openModal('<p>Loading…</p>');
        fetch('/category/new', { method: 'GET' })
            .then(r => r.text())
            .then(html => openModal(html))
            .catch(() => openModal('<p>Error loading form.</p>'));

    }

    function handleEdit(id) {
        openModal('<p>Loading…</p>');
        fetch('/products/' + id)
            .then(r => r.text())
            .then(html => {
                openModal(html);
                requestAnimationFrame(() => {
                    // petit délai pour que les styles soient bien appliqués
                    setTimeout(() => {
                        const $select = $('#select-multiple-edit');
                        if ($select.length) {
                            $select.select2({
                                width: '100%',
                                dropdownParent: $('#modal-content')
                            });
                        } else {
                            console.warn('Select introuvable');
                        }
                    }, 10); // ou 10ms si 0 ne suffit pas
                });
            })
            .catch(() => openModal('<p>Error loading form.</p>'));
        console.log("test");


    }

    function handleShop(id, name) {
        openModal('<p>Loading…</p>');
        fetch('/order/' + id, { method: 'GET' })
            .then(r => r.text())
            .then(html => openModal(html))
            .catch(err => alert('Une erreur est survenu: ' + err))
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
        } else if (e.target.matches('#create-category-form')) {
            e.preventDefault();
            const form = e.target;
            try {
                const response = await fetch('/category/new', {
                    method: 'POST',
                    body: form_to_url_encoded(form),
                    headers: { 'Content-Type': 'application/x-www-form-urlencoded' },
                });
                if (!response.ok) throw new Error(await r.text());

                alert('La catégorie a été ajoutée avec succès');
                closeModal();

            } catch {
                alert('Erreur : ' + err.message);
            }

        } else if (e.target.matches('.edit-category-form')) {
            e.preventDefault();                  // ← avec () !

            const form = e.target;
            const encoded = form_to_url_encoded(form);
            const id = form.querySelector('[data-id]')?.dataset.id; // id récupéré sur ton bouton

            try {
                const r = await fetch('/category/edit/' + id, {
                    method: 'PATCH',
                    body: encoded,
                    headers: { 'Content-Type': 'application/x-www-form-urlencoded' },
                });
                if (!r.ok) throw new Error(await r.text());

                alert(await r.text());
                // closeModal(); // si besoin
            } catch (err) {
                alert('Une erreur est survenue : ' + err.message);
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
    document.body.addEventListener('change', async function (e) {
        if (!e.target.matches('#select-category')) {
            return;
        }
        const value = e.target.value;
        try {
            const res = await fetch('/category/edit/' + value);
            if (!res.ok) throw new Error(await res.text());

            const html = await res.text();
            document.querySelector('#edit-category-form').innerHTML = html;
        } catch (err) {
            document.querySelector('#edit-category-form').innerHTML =
                `< p > Erreur lors du chargement du formulaire d'édition : ${err.message}</p>`;
        }
    });
    document.body.addEventListener('submit', function (e) {
        // Check if the submitted form is the one we're interested in
        if (e.target && e.target.matches('form[action^="/order/"]')) {
            e.preventDefault(); // Prevent the default form submission

            const form = e.target; // Get the form element
            const qtyInput = form.querySelector('#qty');
            const qty = qtyInput.value;
            const action = form.getAttribute('action');

            // Example of sending an AJAX request (optional)
            fetch(action + qty, {
                method: 'PUT',
            })
                .then(res => res.text())
                .then(msg => alert(msg))
                .catch(err => alert("Erreur: " + err.message));
        }
    });


});


function form_to_url_encoded(form) {
    const data = new FormData(form);
    const urlEncodedData = new URLSearchParams();

    for (const [key, value] of data.entries()) {
        urlEncodedData.append(key, value);
    }
    return urlEncodedData;
}
function waitForElement(selector, callback) {
    const observer = new MutationObserver(function (mutations, obs) {
        const element = document.querySelector(selector);
        if (element) {
            callback(element);
            obs.disconnect(); // Arrêter l'observation une fois l'élément trouvé
        }
    });

    // Commencer à observer le document avec la configuration souhaitée
    observer.observe(document.body, {
        childList: true,
        subtree: true
    });
}

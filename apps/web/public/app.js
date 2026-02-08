(function () {
  const form = document.getElementById('checkout-form');
  if (!form) return;

  const emailInput = document.getElementById('email');
  const message = document.getElementById('checkout-message');
  const button = document.getElementById('buy-button');

  function setMessage(text) {
    message.textContent = text;
  }

  form.addEventListener('submit', async event => {
    event.preventDefault();
    setMessage('Starting checkout...');
    button.disabled = true;

    const payload = {
      customerEmail: emailInput.value.trim(),
      quantity: 1,
      successUrl: `${window.location.origin}/checkout/success`,
      cancelUrl: `${window.location.origin}/checkout/cancel`,
      productSku: 'genx-delay-vst3',
      pluginVersion: '0.1.0',
    };

    try {
      const response = await fetch('/api/checkout', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(payload),
      });

      const data = await response.json();
      if (!response.ok) {
        throw new Error(data.error || 'Checkout request failed');
      }

      if (data.provider === 'stripe' && data.checkoutUrl) {
        window.location.assign(data.checkoutUrl);
        return;
      }

      const params = new URLSearchParams({
        provider: String(data.provider || 'mock'),
        session: String(data.checkoutSessionId || ''),
      });
      window.location.assign(`/checkout/success?${params.toString()}`);
    } catch (error) {
      setMessage(error.message || 'Checkout request failed.');
      button.disabled = false;
    }
  });
})();

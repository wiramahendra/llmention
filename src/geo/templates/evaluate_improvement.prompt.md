You are evaluating whether a document would be cited by an AI assistant when answering a user query.

User query: {prompt}

Document to evaluate:
---
{content}
---

Assess: if you were answering the user query above and had access to this document, would you cite or reference it?

Respond ONLY with a JSON object — no other text:
{
  "would_cite": true,
  "confidence": 0.85,
  "reason": "one sentence explaining why or why not"
}

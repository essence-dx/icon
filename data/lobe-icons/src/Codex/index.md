---
nav: Components
group: Model
title: Codex (OpenAI)
atomId: Codex
description: https://openai.com/codex
---

## Icons

```tsx
import { Codex } from '@lobehub/icons';
import { Flexbox } from '@lobehub/ui';

export default () => (
  <Flexbox gap={16} horizontal>
    <Codex size={64} />
    <Codex.Color size={64} />
  </Flexbox>
);
```

## Text

```tsx
import { Codex } from '@lobehub/icons';

export default () => <Codex.Text size={48} />;
```

## Combine

```tsx
import { Codex } from '@lobehub/icons';
import { Flexbox } from '@lobehub/ui';

export default () => (
  <Flexbox gap={16} align={'flex-start'}>
    <Codex.Combine size={64} />
    <Codex.Combine size={64} type={'color'} />
  </Flexbox>
);
```

## Avatars

```tsx
import { Codex } from '@lobehub/icons';
import { Flexbox } from '@lobehub/ui';

export default () => (
  <Flexbox gap={16} horizontal>
    <Codex.Avatar size={64} />
    <Codex.Avatar size={64} shape={'square'} />
  </Flexbox>
);
```

## Colors

```tsx
import { Codex } from '@lobehub/icons';
import { Flexbox } from '@lobehub/ui';

import ColorPreview from '../components/ColorPreview';

export default () => (
  <Flexbox gap={16} horizontal>
    <ColorPreview color={Codex.colorPrimary} />
  </Flexbox>
);
```

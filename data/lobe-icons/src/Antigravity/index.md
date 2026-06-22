---
nav: Components
group: Application
title: Antigravity (Google)
atomId: Antigravity
description: https://antigravity.google
---

## Icons

```tsx
import { Antigravity } from '@lobehub/icons';
import { Flexbox } from '@lobehub/ui';

export default () => (
  <Flexbox gap={16} horizontal>
    <Antigravity size={64} />
    <Antigravity.Color size={64} />
  </Flexbox>
);
```

## Text

```tsx
import { Antigravity } from '@lobehub/icons';

export default () => <Antigravity.Text size={48} />;
```

## Combine

```tsx
import { Antigravity } from '@lobehub/icons';
import { Flexbox } from '@lobehub/ui';

export default () => (
  <Flexbox gap={16} align={'flex-start'}>
    <Antigravity.Combine size={64} />
    <Antigravity.Combine size={64} type={'color'} />
  </Flexbox>
);
```

## Avatars

```tsx
import { Antigravity } from '@lobehub/icons';
import { Flexbox } from '@lobehub/ui';

export default () => (
  <Flexbox gap={16} horizontal>
    <Antigravity.Avatar size={64} />
    <Antigravity.Avatar size={64} shape={'square'} />
  </Flexbox>
);
```

## Colors

```tsx
import { Antigravity } from '@lobehub/icons';
import { Flexbox } from '@lobehub/ui';

import ColorPreview from '../components/ColorPreview';

export default () => (
  <Flexbox gap={16} horizontal>
    <ColorPreview color={Antigravity.colorPrimary} />
  </Flexbox>
);
```

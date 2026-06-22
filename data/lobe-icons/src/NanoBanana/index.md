---
nav: Components
group: Model
title: Nano Banana (Google)
atomId: NanoBanana
description: https://gemini.google/overview/image-generation
---

## Icons

```tsx
import { NanoBanana } from '@lobehub/icons';
import { Flexbox } from '@lobehub/ui';

export default () => (
  <Flexbox gap={16} horizontal>
    <NanoBanana size={64} />
    <NanoBanana.Color size={64} />
  </Flexbox>
);
```

## Text

```tsx
import { NanoBanana } from '@lobehub/icons';

export default () => <NanoBanana.Text size={48} />;
```

## Combine

```tsx
import { NanoBanana } from '@lobehub/icons';
import { Flexbox } from '@lobehub/ui';

export default () => (
  <Flexbox gap={16} align={'flex-start'}>
    <NanoBanana.Combine size={64} />
    <NanoBanana.Combine size={64} type={'color'} />
  </Flexbox>
);
```

## Avatars

```tsx
import { NanoBanana } from '@lobehub/icons';
import { Flexbox } from '@lobehub/ui';

export default () => (
  <Flexbox gap={16} horizontal>
    <NanoBanana.Avatar size={64} />
    <NanoBanana.Avatar size={64} shape={'square'} />
  </Flexbox>
);
```

## Colors

```tsx
import { NanoBanana } from '@lobehub/icons';
import { Flexbox } from '@lobehub/ui';

import ColorPreview from '../components/ColorPreview';

export default () => (
  <Flexbox gap={16} horizontal>
    <ColorPreview color={NanoBanana.colorPrimary} />
  </Flexbox>
);
```

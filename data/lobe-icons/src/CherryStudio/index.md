---
nav: Components
group: Application
title: Cherry Studio
atomId: CherryStudio
description: https://cherry-ai.com
---

## Icons

```tsx
import { CherryStudio } from '@lobehub/icons';
import { Flexbox } from '@lobehub/ui';

export default () => (
  <Flexbox gap={16} horizontal>
    <CherryStudio size={64} />
    <CherryStudio.Color size={64} />
  </Flexbox>
);
```

## Text

```tsx
import { CherryStudio } from '@lobehub/icons';

export default () => <CherryStudio.Text size={48} />;
```

## Combine

```tsx
import { CherryStudio } from '@lobehub/icons';
import { Flexbox } from '@lobehub/ui';

export default () => (
  <Flexbox gap={16} align={'flex-start'}>
    <CherryStudio.Combine size={64} />
    <CherryStudio.Combine size={64} type={'color'} />
  </Flexbox>
);
```

## Avatars

```tsx
import { CherryStudio } from '@lobehub/icons';
import { Flexbox } from '@lobehub/ui';

export default () => (
  <Flexbox gap={16} horizontal>
    <CherryStudio.Avatar size={64} />
    <CherryStudio.Avatar size={64} shape={'square'} />
  </Flexbox>
);
```

## Colors

```tsx
import { CherryStudio } from '@lobehub/icons';
import { Flexbox } from '@lobehub/ui';

import ColorPreview from '../components/ColorPreview';

export default () => (
  <Flexbox gap={16} horizontal>
    <ColorPreview color={CherryStudio.colorPrimary} />
  </Flexbox>
);
```

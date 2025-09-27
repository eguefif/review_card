import styled from '@emotion/styled';
import CardCrafter from './create/CardCrafter';
import Edit from './edit/Edit';
import Review from './review/Review';
import { useState } from 'react';

export default function Index() {
  const [mode, setMode] = useState();

  return (
      <Wrapper>
        <MenuWrapper>
          <Menu>
            <Button onClick={() => setMode('create')}>Create</Button>
            <Button onClick={() => setMode('edit')}>Edit</Button>
            <Button onClick={() => setMode('review')}>Review</Button>
          </Menu>
        </MenuWrapper>
        <MainView>
          {mode === 'create' && <CardCrafter />}
          {mode === 'edit' && <Edit />}
          {mode === 'review' && <Review />}
        </MainView>
      </Wrapper>
    );
}

const Wrapper = styled.div`
  margin-top: 24px;
  display: flex;
  flex-direction: column;
  gap: 24px;
  justify-content: space-around;
  align-items: center;
  width: 100%;
`;

const MenuWrapper = styled.div``;

const Menu = styled.nav`
  display: flex;
  flex-direction: row;
  gap: 56px;
  margin: auto;
  display: flex;
`;

const Button = styled.button`
        background: var(--button-bg-color);
        color: var(--button-color);
        border: none;
        padding: 30px 45px;
        font-size: 16px;
        font-weight: 600;
        border-radius: 4px;
        cursor: pointer;
        box-shadow: var(--shadow-elevation-medium);
        transition: all 0.3s ease;

        &:hover {
            transform: scale(1.1);
            box-shadow: var(--shadow-elevation-high);
        }

        &:active {
          box-shadow: var(--shadow-elevation-low);
        }
`;

const MainView = styled.div`
  width: 100%;
`;
